use std::fs::File;
use std::env;
use deadpool_redis::{Manager, Pool, Runtime, redis::Client as RedisClient};
use redis::{TlsCertificates};

pub async fn connect_redis() -> Pool {
    let root_cert_file_location = env::var("SSL_ROOT_CERT_PATH").ok();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://redis:6379".to_string());


    let client= if root_cert_file_location.is_some() {
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
        RedisClient::build_with_tls(redis_url, tls_certs).expect("Failed to create redis client")
    } else {
        // Fallback to standard connection (non-TLS or system-store TLS)
        RedisClient::open(redis_url).expect("Failed to create redis client")
    };

    let manager = Manager::new(client.get_connection_info().clone()).expect("Failed to create deadpool manager");
        let pool = Pool::builder(manager)
        .max_size(20)
        .runtime(Runtime::Tokio1)
        .build()
        .expect("Failed to create deadpool redis manager");
    return pool;
}