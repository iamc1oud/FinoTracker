

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse}, error::ErrorUnauthorized, Error};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: String,
}

const SECRET_KEY: &str = "12412125";

fn validate_jwt(token: &str) -> Result<Claims, Error> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY.as_ref()), &Validation::new(Algorithm::HS256));

    match token_data {
        Ok(data) => Ok(data.claims),
        Err(_) => Err(ErrorUnauthorized("Unauthorized")),
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}