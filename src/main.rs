use actix_web::{get, web, App, HttpServer};
mod api;
mod config;

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
    let db_url = &data.database_url;
    format!("Database URL is here: {db_url}!")
}
