#[macro_use]
extern crate log;

use monitor::{
    collector::{self, collector_server},
    config,
    Continue,
    create_shell_checks,
    // create_shell_metrics,
    LogStore,
    MetricStore,
    scheduler::Scheduler,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()))
        .format_timestamp_micros()
        .init();

    let config = load_config();
    trace!("rudano config=\n{}", rudano::to_string_pretty(&config).unwrap());

    let ls = Arc::new(Mutex::new(LogStore::new()));
    let ms = Arc::new(Mutex::new(MetricStore::new()));
    // let n = Arc::new(Mutex::new(Notifier::new()));
    let sched = Arc::new(Mutex::new(Scheduler::new()));

    create_shell_checks(&config.shell_checks, &ls, &ms, &sched);

    // connect_all_checks_to_notifier(&ms, &n);

    sched.lock().unwrap().spawn();

    let addr: std::net::SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let collector_service = CollectorService {
        metric_store: ms.clone(),
    };
    let tls_config = tls_config(&config).expect("Ok TLS config");
    info!("Listening on {}://{}",
          match tls_config {
              Some(_) => "https",
              None => "http",
          }, addr);
    let sb = tonic::transport::Server::builder();
    let mut sb = match tls_config {
        Some(tls) => sb.tls_config(tls).expect("tls config OK"),
        None => sb,
    };
    sb.add_service(collector::collector_server::CollectorServer::new(collector_service))
      .serve(addr)
      .await.unwrap();
}

fn load_config() -> config::Collector {
    // Panic on error because without config we can't continue.

    let exe_path = std::env::current_exe().expect("Expect to retrieve current exe path");
    let exe_dir = exe_path.parent().expect("Expect to retrieve current exe parent");
    let config_path = exe_dir.join("collector.rudano");

    let config_str = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("Error reading the config file from `{:?}': {}",
                                  &config_path, e));
    rudano::from_str(&config_str).expect("Config file to parse")
}

fn tls_config(config: &config::Collector
) -> Result<Option<tonic::transport::ServerTlsConfig>, Box<dyn std::error::Error>> {
    let identity_config = match config.server_tls_identity.as_ref() {
        None => return Ok(None),
        Some(idc) => idc,
    };
    let identity = identity_config.load().expect("Load server TLS identity");
    let tls_config = tonic::transport::ServerTlsConfig::new()
        .identity(identity);
    let tls_config = match config.client_tls_ca_cert.as_ref() {
        Some(client_ca_cert) => tls_config.client_ca_root(client_ca_cert.load()?),
        None => tls_config,
    };
    Ok(Some(tls_config))
}

struct CollectorService {
    metric_store: Arc<Mutex<MetricStore>>,
}

#[tonic::async_trait]
impl collector_server::Collector for CollectorService {
    async fn get_metrics(
        &self,
        request: tonic::Request<collector::GetMetricsRequest>,
    ) -> Result<tonic::Response<collector::MetricsReply>, tonic::Status>
    {
        trace!("CollectorService::get_metrics");
        trace!("request remote_addr={:?}, \n  {:#?}", request.remote_addr(), request);
        let metrics = self.metric_store.lock().unwrap().query_all();
        let metrics = metrics.iter().map(|m| m.as_protobuf())
                             .collect::<Result<Vec<collector::Metric>, String>>();
        if let Err(e) = metrics {
            error!("metrics as_protobuf error: {}", e);
            return Err(tonic::Status::internal("Internal error."));
        }
        let metrics = metrics.unwrap();
        trace!("num_metrics = {}", metrics.len());
        Ok(tonic::Response::new(collector::MetricsReply {
            metrics,
        }))
    }

    type StreamMetricsStream =
        tokio::sync::mpsc::Receiver<Result<collector::Metric, tonic::Status>>;
    async fn stream_metrics(
        &self,
        request: tonic::Request<collector::StreamMetricsRequest>,
    ) -> Result<tonic::Response<Self::StreamMetricsStream>, tonic::Status>
    {
        trace!("CollectorService::stream_metrics");
        trace!("request remote_addr={:?}, \n  {:#?}", request.remote_addr(), request);

        // Lock the metric store until we've sent all initial values and registered our listener
        let mut lock = self.metric_store.lock().unwrap();
        let metrics = lock.query_all()
                          .iter().map(|m| m.as_protobuf())
                          .collect::<Result<Vec<collector::Metric>, String>>();
        if let Err(e) = metrics {
            error!("metrics as_protobuf error: {}", e);
            return Err(tonic::Status::internal("Internal error."));
        }
        let metrics = metrics.unwrap();
        let (mut tx, rx) = tokio::sync::mpsc::channel(metrics.len() + 10);
        for m in metrics.into_iter() {
            tx.try_send(Ok(m)).expect("channel to have capacity for all initial values");
        }
        trace!("CollectorService::stream_metrics sent initial values");
        // Wrap in a mutex to keep the borrow checker happy that we're using a
        // &mut method on a captured variable in an Fn closure.
        let tx = Mutex::new(tx);
        lock.update_signal_for_all().connect(move |metric| {
            let metric_proto = match metric.as_protobuf() {
                Err(e) => {
                    error!("stream_metrics: failed to encode as protobuf: {}", e);
                    return Continue::Continue;
                },
                Ok(m) => m,
            };
            match tx.lock().unwrap().try_send(Ok(metric_proto)) {
                Err(tokio::sync::mpsc::error::TrySendError::Full(_)) =>
                    error!("stream_metrics: channel full key={}",
                           metric.key().display_name()),
                Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                    debug!("stream_metrics: channel closed");
                    return Continue::Disconnect;
                }
                Ok(()) =>
                    trace!("stream_metrics: sent metric key=`{}'", metric.key().display_name()),
            };
            Continue::Continue
        });
        Ok(tonic::Response::new(rx))
    }
}
