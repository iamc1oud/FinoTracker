use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{config, models::user::user::User};

#[derive(Serialize, Deserialize)]
struct Login {
    login_type: String,
}

#[get("/login")]
pub async fn login(dbConn: web::Data<config::database::Database>) -> impl Responder {
     let mut account = User::new();

    account.email = Some("Ajay".to_string());
    
    let _ = dbConn.user.insert_one(account).await;

    HttpResponse::Ok().json(Login {
        login_type: String::from("IOS"),
    })
}
