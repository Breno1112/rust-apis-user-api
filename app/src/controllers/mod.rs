pub mod healthcheck;
pub mod user_controller;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
        .configure(healthcheck::config)
        .configure(user_controller::config)
    );
}