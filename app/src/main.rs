use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::env;

mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let ssl_context = generate_ssl_context();

    if ssl_context.is_some() {
        HttpServer::new(|| {
        App::new()
            .configure(controllers::init)
        })
        .bind_openssl(("0.0.0.0", 8081), ssl_context.unwrap())?
        .run()
        .await
    } else {
        HttpServer::new(|| {
        App::new()
            .configure(controllers::init)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    }
}

fn generate_ssl_context() -> Option<SslAcceptorBuilder> {
    let key_file_location = env::var("SSL_KEY_PATH").unwrap_or("null".to_string());
    let cert_file_location = env::var("SSL_CERT_PATH").unwrap_or("null".to_string());

    if "null" != key_file_location && "null" != cert_file_location {
        let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        ssl_builder.set_private_key_file(key_file_location, SslFiletype::PEM).unwrap();
        ssl_builder.set_certificate_chain_file(cert_file_location).unwrap();
        return Some(ssl_builder);
    }
    return None;
}