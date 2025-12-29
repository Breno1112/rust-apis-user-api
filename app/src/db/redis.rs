use std::fs::File;
use std::env;
use redis::{Client, TlsCertificates};

pub async fn connect_redis() -> redis::RedisResult<Client> {
    let root_cert_file_location = env::var("SSL_ROOT_CERT_PATH").ok();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string());

    if root_cert_file_location.is_some() {
        // 1. Read the certificate bytes
        
        let mut file = File::open(&root_cert_file_location.unwrap()).expect("Failed to open root cert file");
        let mut root_cert_bytes = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut root_cert_bytes).expect("Failed to read cert");

        // 2. Wrap it in the Redis TlsCertificates struct
        let tls_certs = TlsCertificates {
            client_tls: None, // No client-side auth (mTLS) needed here
            root_cert: Some(root_cert_bytes),
        };

        // 3. IMPORTANT: Use build_with_tls to link the config to the client
        // The URL MUST start with rediss:// for this to work
        redis::Client::build_with_tls(redis_url, tls_certs)
    } else {
        // Fallback to standard connection (non-TLS or system-store TLS)
        redis::Client::open(redis_url)
    }
}