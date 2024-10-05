use actix_web::{
    cookie::time::Date, http::header, middleware::Logger, HttpResponse
};
use mongodb::bson::{oid::ObjectId, DateTime};

use crate::{api::auth::dtos::LoginBodyDTO, common::errors::types::ApiResponse, models::user::user::{MongoUserRepository, User, UserRepository}};

pub struct LoginUsecase {
    user_repository: MongoUserRepository
}

impl LoginUsecase {
    pub fn new(user_repository: MongoUserRepository) -> Self {
        LoginUsecase { user_repository }
    }

    pub async fn execute(&self, command: LoginBodyDTO) -> HttpResponse {
        let email_record = self.user_repository.find_by_email(&command.email).await;
        match email_record {
            Ok(result) => {
                if result.is_some() {
                    let response = ApiResponse::success("Email found", result);

                    return HttpResponse::Ok().json(response);
                }

                // Create new email record
                let _ = self.user_repository.create(User{
                    _id: Some(ObjectId::new()),
                    country_code: None,
                    first_name: None,
                    last_name: None,
                    profile_picture: None,
                    phone_number: None,
                    created_at: Some(DateTime::now()),
                    updated_at: Some(DateTime::now()),
                    email:  Some(command.email.to_string()),
                }).await;
            },
            Err(err) => {
                println!("Error: {}", err);
            },
        }

        let response = ApiResponse::success("User data retrieved", command);

        return HttpResponse::Ok()
            .insert_header((header::CACHE_CONTROL, "no-store"))
            .json(response);
    }
}
