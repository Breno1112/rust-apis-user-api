use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslMethod, SslFiletype};
use std::env;

pub fn generate_ssl_context() -> Option<SslAcceptorBuilder> {
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