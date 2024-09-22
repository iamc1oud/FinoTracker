mod api;
mod config;
mod models;

use actix_web::{web, App, HttpServer};



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = config::database::Database::init().await;

    println!("DB Connected");

    let db_data = web::Data::new(db);

    HttpServer::new(move ||{
        App::new()
            .app_data(db_data.clone())
            .configure(api::init_routes)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}