use std::env;

use mongodb::options::TlsOptions;

pub async fn create_mongo_client() -> mongodb::Client {

    let mongodb_url = env::var("MONGODB_URL").unwrap_or("mongodb://mongodb:20000".to_string());
    let mongodb_cert_path = env::var("MONGODB_CERT_PATH").ok();
    let mongodb_username = env::var("MONGODB_USERNAME").unwrap_or("adm".to_string());
        let mongodb_password = env::var("MONGODB_PASSWORD").unwrap_or("pass".to_string());

    let mut options = mongodb::options::ClientOptions::parse(mongodb_url).await.unwrap();
    if mongodb_cert_path.is_some() {
        options.credential = Some(
            mongodb::options::Credential::builder()
            .username(Some(mongodb_username))
            .password(Some(mongodb_password))
            .source(Some("admin".to_string()))
            .build()
        );
        options.tls = Some(
            mongodb::options::Tls::Enabled(TlsOptions::builder()
            .ca_file_path(std::path::PathBuf::from(mongodb_cert_path.unwrap()))
            .build())
        );
        mongodb::Client::with_options(options).unwrap()
    } else {
        mongodb::Client::with_options(options).unwrap()
    }
}