#[macro_use]
extern crate log;

mod auth;
mod session;

use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;
use crate::{
    auth::Auth,
    session::Sessions,
};
use monitor::{
    // BoxError,
    config,
    log_store::{LogStore},
    metric_store::{Metric, MetricStore},
    remote,
};
use serde::Deserialize;
use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct AppContext {
    auth: Arc<Auth>,
    config: config::Web,
    log_store: Arc<Mutex<LogStore>>,
    metric_store: Arc<Mutex<MetricStore>>,
    remotes: Arc<remote::Remotes>,
    sessions: Arc<Sessions>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()))
        .format_timestamp_micros()
        .init();

    let config = load_config();
    trace!("rudano config=\n{}", rudano::to_string_pretty(&config).unwrap());

    let ls = Arc::new(Mutex::new(LogStore::new()));
    let ms = Arc::new(Mutex::new(MetricStore::new()));
    let remotes = remote::Remotes::from_configs(&config.remote_syncs)
                                  .expect("RemoteSync configs OK");
    let remotes = Arc::new(remotes);
    remote::spawn_sync_jobs(&remotes, &ls, &ms);

    let ctx = actix_web::web::Data::new(AppContext {
        auth: Arc::new(Auth::new()),
        config: config.clone(),
        log_store: ls,
        metric_store: ms,
        remotes,
        sessions: Arc::new(Sessions::new()),
    });

    let exe_path = std::env::current_exe().expect("Expect to retrieve current exe path");
    let exe_dir = exe_path.parent().expect("Expect to retrieve current exe parent");
    let web_static_path = exe_dir.join("web-static");
    assert!(web_static_path.exists(), "path for /static must exist at `{:?}'", web_static_path);

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                actix_web::middleware::errhandlers::ErrorHandlers::new()
                    .handler(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, render_500),
            )
            .wrap(
                actix_web::middleware::errhandlers::ErrorHandlers::new()
                    .handler(actix_web::http::StatusCode::NOT_FOUND, render_404),
            )
            .app_data(ctx.clone())
            .route("/", actix_web::web::get().to(index))
            .route("/login", actix_web::web::get().to(login_get))
            .route("/login", actix_web::web::post().to(login_post))
            .route("/logout", actix_web::web::get().to(logout_get))
            .service(actix_files::Files::new("/static", web_static_path.clone())
                     .use_last_modified(true)
                     .show_files_listing())
    });

    match config.server_tls_identity {
        None => server.bind(config.listen_addr)?
                      .run()
                      .await?,
        Some(tls) => {
            let mut tls_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
            tls_config.set_single_cert(
                load_cert_chain(&tls.cert_path),
                load_private_key(&tls.key_path),
            ).unwrap();
            tls_config.set_protocols(&["h2".as_bytes().to_vec()]);
            server.bind_rustls(config.listen_addr, tls_config)?
                  .run()
                  .await?
        }
    };
    Ok(())
}

fn load_config() -> config::Web {
    // Panic on error because without config we can't continue.

    let exe_path = std::env::current_exe().expect("Expect to retrieve current exe path");
    let exe_dir = exe_path.parent().expect("Expect to retrieve current exe parent");
    let config_path = exe_dir.join("web.rudano");

    let config_str = std::fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("Error reading the config file from `{:?}': {}",
                                  &config_path, e));
    rudano::from_str(&config_str).expect("Config file to parse")
}

fn render_404<B>(mut res: actix_web::dev::ServiceResponse<B>
) -> actix_web::Result<actix_web::middleware::errhandlers::ErrorHandlerResponse<B>> {
    res.response_mut()
       .headers_mut()
       .insert(actix_web::http::header::CONTENT_TYPE,
               actix_web::http::HeaderValue::from_static("text/plain"));
    let res = res.map_body::<_, B>(|_head, _body|
                 actix_web::dev::ResponseBody::Other(
                     actix_web::dev::Body::Bytes(
                         actix_web::web::Bytes::from("Not found"))));
    Ok(actix_web::middleware::errhandlers::ErrorHandlerResponse::Response(res))
}

fn render_500<B>(mut res: actix_web::dev::ServiceResponse<B>
) -> actix_web::Result<actix_web::middleware::errhandlers::ErrorHandlerResponse<B>> {
    res.response_mut()
       .headers_mut()
       .insert(actix_web::http::header::CONTENT_TYPE,
               actix_web::http::HeaderValue::from_static("text/plain"));
    let res = res.map_body::<_, B>(|_head, _body|
                 actix_web::dev::ResponseBody::Other(
                     actix_web::dev::Body::Bytes(
                         actix_web::web::Bytes::from("Internal server error"))));
    Ok(actix_web::middleware::errhandlers::ErrorHandlerResponse::Response(res))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    metrics: Vec<Metric>,
}

async fn index(ctx: actix_web::web::Data<AppContext>, req: HttpRequest
) -> actix_web::Result<impl Responder> {
    // If not logged in, redirect to "/login".
    let session = ctx.sessions.get(&req);
    if session.is_none() {
        let mut res = HttpResponse::SeeOther(); // 303
        res.header(actix_web::http::header::LOCATION, "/login");
        return Ok(res.finish())
    }

    // Authenticated.
    let metrics = ctx.metric_store.lock().unwrap().query_all();
    let mut res = HttpResponse::Ok();
    res.content_type("text/html");
    let res = res.body((IndexTemplate {
        metrics,
    }).render().unwrap());
    Ok(res)
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    message: Option<&'a str>,
}

async fn login_get(_ctx: actix_web::web::Data<AppContext>, _req: HttpRequest
) -> actix_web::Result<impl Responder> {
    let mut res = HttpResponse::Ok();
    res.content_type("text/html");
    let res = res.body((LoginTemplate { message: None })
                           .render().unwrap());
    Ok(res)
}

#[derive(Deserialize)]
struct LoginPostArgs {
    username: String,
    password: String,
}

async fn login_post(
    ctx: actix_web::web::Data<AppContext>,
    _req: HttpRequest,
    form: actix_web::web::Form<LoginPostArgs>,
) -> actix_web::Result<impl Responder> {
    let auth_token = ctx.auth.login(&form.username, &form.password);

    if auth_token.is_some() {
        let mut res = HttpResponse::SeeOther(); // 303
        if let Err(e) = ctx.sessions.login(&mut res) {
            error!("Error calling Sessions::login: {}", e);
            return Err(().into());
        }
        res.header(actix_web::http::header::LOCATION, "/");
        Ok(res.finish())
    } else {
        assert!(auth_token.is_none());

        let mut res = HttpResponse::Unauthorized(); // 401
        res.content_type("text/html");
        let res = res.body((LoginTemplate { message: Some("Bad username or password") })
                           .render().unwrap());
        Ok(res)
    }
}

async fn logout_get(ctx: actix_web::web::Data<AppContext>, req: HttpRequest
) -> actix_web::Result<impl Responder> {
    let mut res = HttpResponse::Ok();
    res.content_type("text/html");

    ctx.sessions.logout(&req, &mut res);
    let res = res.body((LoginTemplate { message: Some("Logged out") })
                           .render().unwrap());
    Ok(res)
}

fn load_cert_chain(filename: &str) -> Vec<rustls::Certificate> {
    let file = File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(file);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let rsa_keys = {
        let keyfile = File::open(filename)
            .expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::rsa_private_keys(&mut reader)
            .expect("file contains invalid rsa private key")
    };

    let pkcs8_keys = {
        let keyfile = File::open(filename)
            .expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader)
            .expect("file contains invalid pkcs8 private key (encrypted keys not supported)")
    };

    // prefer to load pkcs8 keys
    if !pkcs8_keys.is_empty() {
        pkcs8_keys[0].clone()
    } else {
        assert!(!rsa_keys.is_empty());
        rsa_keys[0].clone()
    }
}
