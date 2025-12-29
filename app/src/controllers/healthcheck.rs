use actix_web::{web, get, HttpResponse, Responder};

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/healthcheck")
        .service(status)
    );
}