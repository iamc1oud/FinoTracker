
use actix_web::web::{self};
use actix_web::{post, web::Json, HttpResponse, Responder};
use validator::Validate;

use crate::api::auth::dtos::{CreateAccountDTO, LoginBodyDTO};
use crate::api::auth::usecases::create_account_usecase::CreateAccountUsecase;
use crate::api::auth::usecases::login_usecase::LoginUsecase;
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

#[post("/account/create")]
pub async fn create_account(body: Json<CreateAccountDTO>, db: web::Data<Database>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let usecase = CreateAccountUsecase::new(db.account.clone());

            return usecase.execute(CreateAccountDTO {
                email: body.email.to_string()
            }).await;
        },
        Err(err) => HttpResponse::BadRequest().json(err.errors().clone()) 
    }
}
