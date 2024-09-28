use actix_web::web::{self, ServiceConfig};

pub mod auth_controller;
pub mod dtos;
pub mod middlewares;
pub mod services;
pub mod usecases;
pub mod e2e;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    // Version 1
    cfg.service(web::scope("/api/v1").service(auth_controller::login));
}