use actix_web::web;

pub mod auth;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Register service routes here.
    cfg.service(web::scope("/api").service(auth::login),);

}