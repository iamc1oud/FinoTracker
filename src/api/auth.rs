use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config, models::user::user::User};

#[derive(Serialize, Deserialize)]
struct Login {
    login_type: String,
    account_details: Option<String>,
}

#[get("/login")]
pub async fn login(dbConn: web::Data<config::database::Database>) -> impl Responder {
    let mut account = User::new();

    let created = dbConn.user.insert_one(account).await;

    match created {
        Ok(success) => {
            return HttpResponse::Ok().json(Login {
                login_type: "Hello world".to_string(),
                account_details: Some("account._id.to_hex()".to_string()),
            });
        }
        Err(err) => {
            return  HttpResponse::BadRequest().json(err.clone().to_string());
        }
    }
}
