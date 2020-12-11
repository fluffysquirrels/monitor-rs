#[macro_use]
extern crate log;

use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;
use monitor::{
    // BoxError,
    config,
};
use std::{
    fs::File,
    io::BufReader,
};

fn config() -> config::Web {
    config::Web {
        listen_addr: "127.0.0.1:8443".to_owned(),
        remote_syncs: vec![],
        server_tls_identity: Some(config::TlsIdentity {
            cert_path: "/home/alex/Code/rust/monitor/cert/ok/mf.fullchain".to_owned(),
            key_path: "/home/alex/Code/rust/monitor/cert/ok/mf.key".to_owned(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()))
        .format_timestamp_micros()
        .init();

    let config = config();
    trace!("rudano config=\n{}", rudano::to_string_pretty(&config).unwrap());

    let server = actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/", actix_web::web::get().to(root))
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

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    mood: &'a str,
    task: &'a str,
}

async fn root(_req: HttpRequest) -> impl Responder {
    let mut res = HttpResponse::Ok();
    res.content_type("text/html");
    res.body((HelloTemplate {
        mood: "determined",
        task: "<html>",
    }).render().unwrap())
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
