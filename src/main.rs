mod api;
mod config;
mod models;
mod security;
mod common;

use actix_web::{middleware::{self}, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = config::database::Database::init().await;

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-API-Version", "1.0.0")))
            .app_data(db_data.clone())
            .configure(api::auth::register_routes)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
