#[macro_use]
extern crate log;

mod auth;
mod handlers;
mod session;

mod web_socket_types {
    //! Protobuf types for the WebSocket endpoint

    tonic::include_proto!("monitor_web_socket");
}

use crate::{
    auth::Auth,
    session::Sessions,
};
use monitor::{
    // BoxError,
    config,
    log_store::LogStore,
    metric_store::MetricStore,
    remote,
};
use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct AppContext {
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
        sessions: Arc::new(Sessions::with_secure(config.server_tls_identity.is_some())),
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
            .route("/", actix_web::web::get().to(handlers::metrics::metrics_get))
            .route("/login", actix_web::web::get().to(handlers::auth::login_get))
            .route("/login", actix_web::web::post().to(handlers::auth::login_post))
            .route("/logout", actix_web::web::get().to(handlers::auth::logout_get))
            .route("/ws/", actix_web::web::get().to(handlers::websocket::websocket_get))
            .service(actix_files::Files::new("/static", web_static_path.clone())
                     .use_last_modified(true)
                     .show_files_listing()
                     .disable_content_disposition())
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
