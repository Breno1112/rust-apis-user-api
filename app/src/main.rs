use actix_web::{App, HttpServer, web};

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

    if ssl_context.is_some() {
        HttpServer::new(move || {
        App::new()
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
            .app_data(redis_pool.clone())
            .app_data(mongo_client.clone())
            .configure(controllers::init)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    }
}