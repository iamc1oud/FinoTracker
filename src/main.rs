mod api;
mod config;
mod models;
mod security;

use actix_web::{dev::Service, web, App, HttpServer};
use futures_util::FutureExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = config::database::Database::init().await;

    println!("DB Connected");

    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                
                srv.call(req).map(|res| res)
            })
            .app_data(db_data.clone())
            .configure(api::init_routes)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
