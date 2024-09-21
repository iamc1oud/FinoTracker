use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{config, models::accounts::Accounts};

#[derive(Serialize, Deserialize)]
struct Login {
    login_type: String,
}

#[get("/login")]
pub async fn login(dbConn: web::Data<config::database::Database>) -> impl Responder {
     let mut account = Accounts::new();

    account.email = Some("Ajay".to_string());
    
    let _ = dbConn.accounts.insert_one(account).await;

    HttpResponse::Ok().json(Login {
        login_type: String::from("IOS"),
    })
}
