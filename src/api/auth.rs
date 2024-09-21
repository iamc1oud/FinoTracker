use actix_web::{get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Login {
    login_type: String,
}

#[get("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().json(Login{
        login_type:  String::from("IOS")
    })
}
