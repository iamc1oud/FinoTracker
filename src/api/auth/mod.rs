use actix_web::web::{self, ServiceConfig};

pub mod auth_controller;
pub mod dtos;
pub mod e2e;
pub mod middlewares;
pub mod services;
pub mod usecases;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(auth_controller::login)
            .service(auth_controller::create_account),
    );
}
