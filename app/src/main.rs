use actix_web::{App, HttpServer};

mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(controllers::init)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}