use actix_web::{Error, HttpResponse};
use mongodb::bson::{doc, to_document};

use crate::{api::auth::dtos::CreateAccountDTO, common::errors::types::ApiResponse, models::auth::account::{AccountRepository, MongoAccountRepository}};

pub struct CreateAccountUsecase {
    account_repository: MongoAccountRepository
}

impl CreateAccountUsecase {
    pub fn new(account_repository: MongoAccountRepository) -> Self {
        CreateAccountUsecase { account_repository }
    }

    pub async fn execute(&self, command: CreateAccountDTO) -> HttpResponse {
        let created = self.account_repository.create(&command.email).await;

        match created {
            Ok(result) => {
                let  response= ApiResponse::success("Created new account", result.clone());

                return HttpResponse::Accepted().json(response);
            },
            Err(err) => {
                return HttpResponse::BadRequest().json(ApiResponse::<()>::error(&err.to_string(), None))
            },
        }
    }
}