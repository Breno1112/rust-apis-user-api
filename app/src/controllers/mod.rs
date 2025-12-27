pub mod healthcheck;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
        .configure(healthcheck::config)
    );
}