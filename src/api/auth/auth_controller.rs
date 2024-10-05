
use actix_web::http::StatusCode;
use actix_web::web::{self};
use actix_web::{post, web::Json, HttpResponse, Responder};
use validator::Validate;

use crate::api::auth::dtos::LoginBodyDTO;
use crate::api::auth::usecases::login::login_usecase::LoginUsecase;
use crate::config::database::Database;

#[post("/account/login")]
pub async fn login(body: Json<LoginBodyDTO>, db: web::Data<Database>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let usecase = LoginUsecase::new(db.user.clone());
            usecase.execute(LoginBodyDTO{email: body.email.to_string(), password: body.password.to_string()}).await
        },
        Err(err) => HttpResponse::BadRequest().json(err.errors().clone()),
    }
}
