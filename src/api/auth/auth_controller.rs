use actix_web::{http::header, post, web::Json, HttpResponse, Responder};
use validator::Validate;

use crate::api::auth::dtos::LoginBodyDTO;
use crate::api::auth::usecases::login::login_usecase::Login;

#[post("/account/login")]
pub async fn login(body: Json<LoginBodyDTO>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) =>
        {
            Login::execute(body.into_inner()).await
        }
        Err(err) => HttpResponse::BadRequest().json(err)
    }
}
