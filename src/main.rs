use actix_web::{get, web, App, HttpServer};
mod config;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = config::load_config().expect("Failed to load config");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config::Config {
                database_url: config.database_url.clone(),
            }))
            .configure(api::init_routes)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index(data: web::Data<config::Config>) -> String {
    let db_url = &data.database_url; // <- get app_name
    format!("Database URL: {db_url}!") // <- response with app_name
}
