use actix::AsyncContext;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;
use crate::{
    AppContext,
    session,
    web_socket_types,
};
use monitor::{
    Continue,
    metric_store::{Metric},
    monitor_core_types,
};
use prost::Message;

pub async fn websocket_get(
    ctx: actix_web::web::Data<AppContext>,
    req: HttpRequest,
    stream: actix_web::web::Payload
) -> Result<HttpResponse, actix_web::Error> {
    let peer_addr = req.peer_addr().unwrap();
    let log_ctx = format!("WSA peer={}", peer_addr);

    let sess = ctx.sessions.get_with_req(&req);
    info!("{} starting, authed = {}", log_ctx, sess.is_some());

    let sess = match sess {
        None => {
            // Not authenticated.
            let mut res = HttpResponse::Unauthorized(); // 401
            res.content_type("text/plain");
            let res = res.body("Unauthorized");
            return Ok(res);
        },
        Some(sess) => sess,
    };

    // Authenticated.

    info!("{} starting actor", log_ctx);
    let resp = ws::start(WebSocketActor {
        ctx: (**ctx).clone(),
        peer_addr,
        session: sess,
    }, &req, stream);
    resp
}

struct WebSocketActor {
    ctx: AppContext,
    peer_addr: std::net::SocketAddr,

    #[allow(dead_code)]
    session: session::Session,
}

impl actix::Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let log_ctx = format!("WSA peer={}", self.peer_addr);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Binary(bin)) => {
                let msg = match web_socket_types::ToServer::decode(&*bin) {
                    Err(e) => {
                        warn!("{} Decode error: {}", log_ctx, e);
                        return;
                    },
                    Ok(msg) => msg,
                };
                self.handle_incoming(msg, ctx);
            },
            Ok(ws::Message::Close(reason)) => {
                info!("{} Closed reason={:?}", log_ctx, reason);
            },
            Ok(ws::Message::Text(_)) => warn!("{} Unexpected text message", log_ctx),
            Ok(_) => {},
            Err(e) => {
                warn!("{} Error: {}", log_ctx, e)
            }
        }
    }
}

struct MetricUpdateMessage {
    metric: Metric,
}

impl actix::Message for MetricUpdateMessage {
    type Result = ();
}

impl actix::Handler<MetricUpdateMessage> for WebSocketActor {
    type Result = ();
    fn handle(&mut self, msg: MetricUpdateMessage, ctx: &mut Self::Context) -> Self::Result {
        let log_ctx = format!("WSA peer={}", self.peer_addr);

        let m_proto = match msg.metric.to_remote(&self.ctx.config.host_name) {
            Ok(p) => p,
            Err(e) => {
                error!("{} Error encoding Metric to protobuf: {}", log_ctx, e);
                return;
            }
        };
        let ws_msg = web_socket_types::ToClient {
            msg: Some(web_socket_types::to_client::Msg::MetricsUpdate(
                web_socket_types::MetricsUpdate {
                    metrics: vec![m_proto],
                })),
        };
        if let Err(e) = self.send(ctx, &ws_msg) {
            error!("{} Error sending MetricsUpdate: {}", log_ctx, e);
            return ();
        }
    }
}

impl WebSocketActor {
    fn handle_incoming(
        &mut self,
        msg: web_socket_types::ToServer,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let log_ctx = format!("WSA peer={}", self.peer_addr);
        trace!("{} Incoming={:?}", log_ctx, &msg);
        let msg = match msg.msg {
            Some(msg) => msg,
            None => {
                error!("{} Missing ToServer.msg", log_ctx);
                return;
            }
        };

        match msg {
            web_socket_types::to_server::Msg::SubscribeToMetrics(_sub) => {
                let addr = ctx.address();

                let mut ms_lock = self.ctx.metric_store.lock().unwrap();
                let log_ctxc = log_ctx.clone();
                ms_lock.update_signal_for_all().connect(move |m| {
                    let msg = MetricUpdateMessage {
                        metric: m,
                    };
                    if let Err(e) = addr.try_send(msg) {
                        use actix::prelude::SendError;
                        match e {
                            SendError::Full(_msg) => {
                                error!("{} Error sending metric update message: queue full",
                                       log_ctxc);
                            },
                            SendError::Closed(_msg) => {
                                info!("{} Sending metric update message: recipient closed",
                                      log_ctxc);
                                return Continue::Disconnect;
                            },
                        }
                    }
                    Continue::Continue
                });
                let metrics = ms_lock.query_all();
                drop(ms_lock);

                let protos = metrics.iter().map(|m| m.to_remote(&self.ctx.config.host_name))
                                    .collect::<Result<Vec<monitor_core_types::Metric>, _>>();
                let protos = match protos {
                    Err(e) => {
                        error!("{} Error encoding Metrics to protobuf: {}", log_ctx, e);
                        return;
                    }
                    Ok(ps) => ps,
                };
                let msg = web_socket_types::ToClient {
                    msg: Some(web_socket_types::to_client::Msg::MetricsUpdate(
                        web_socket_types::MetricsUpdate {
                            metrics: protos,
                        })),
                };
                if let Err(e) = self.send(ctx, &msg) {
                    error!("{} Error sending MetricsUpdate: {}", log_ctx, e);
                    return;
                }
            },
            web_socket_types::to_server::Msg::Ping(ping) => {
                let pong = web_socket_types::ToClient {
                    msg: Some(web_socket_types::to_client::Msg::Pong(
                        web_socket_types::Pong {
                            payload: ping.payload
                        }))
                };

                if let Err(e) = self.send(ctx, &pong) {
                    error!("{} Error sending pong: {}", log_ctx, e);
                }
            },
        }
    }

    fn send(
        &self,
        ctx: &mut ws::WebsocketContext<Self>,
        msg: &web_socket_types::ToClient
    ) -> Result<(), String> {
        let log_ctx = format!("WSA peer={}", self.peer_addr);
        let mut buf = vec![];
        if let Err(e) = msg.encode(&mut buf) {
            let err_msg = format!("to_client::Msg encode error: {}", e);
            return Err(err_msg);
        }
        ctx.binary(buf);
        trace!("{} Sent message to client", log_ctx);
        Ok(())
    }
}
