use actix_web::{App, HttpServer, middleware::Logger, web};

mod controllers;
mod config;
mod db;
mod domain;
mod mappers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ssl_context = config::generate_ssl_context();
    let redis_pool = web::Data::new(db::redis::connect_redis().await);
    let mongo_client = web::Data::new(db::mongodb::create_mongo_client().await);

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    if ssl_context.is_some() {
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::new("\nIP: %a\nUser-Agent: %{User-Agent}i\nStatusCode: %s\nTotal time(ms): %D"))
                .app_data(redis_pool.clone())
            .app_data(mongo_client.clone())
            .configure(controllers::init)
        })
        .bind_openssl(("0.0.0.0", 8081), ssl_context.unwrap())?
        .run()
        .await
    } else {
        HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("\nIP: %a\nUser-Agent: %{User-Agent}i\nStatusCode: %s\nTotal time(ms): %D"))
            .app_data(redis_pool.clone())
            .app_data(mongo_client.clone())
            .configure(controllers::init)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    }
}