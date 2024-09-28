use actix_web::{
    http::header,
    HttpResponse,
};

use crate::{api::auth::dtos::LoginBodyDTO, common::errors::types::ApiResponse};

pub struct Login {}

impl Login {
    pub async fn execute(command: LoginBodyDTO) -> HttpResponse {
        // Call the login usecase
        let response = ApiResponse::success("User data retrieved", command);

        return HttpResponse::Ok()
            .insert_header((header::CACHE_CONTROL, "no-store"))
            .json(response);
    }
}
