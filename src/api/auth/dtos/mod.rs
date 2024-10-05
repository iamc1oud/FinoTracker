extern crate lazy_static;

use std::env;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
    static ref RE_USER_NAME: Regex = Regex::new(r"^[a-zA-Z0-9]{6,}$").unwrap();
}

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginBodyDTO {
    // #[validate(regex(
    //     path = "RE_USER_NAME",
    //     message = "Username must number and alphabets only and must be 6 characters long"
    // ))]
    pub email: String,

    #[validate(custom(
        function = "validate_password",
        message = "Must Contain At Least One Upper Case, Lower Case and Number"
    ))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateAccountDTO {
    #[validate(email)]
    pub email: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }

    let password_min_length_env = env::var("PASSWORD_MIN_LENGTH").expect("PASSWORD_MIN_LENGTH is not set in the environment");

    let password_min_length: usize = password_min_length_env
        .parse()
        .expect("PASSWORD_MIN_LENGTH is not a valid number");

    if !has_whitespace
        && has_upper
        && has_lower
        && has_digit
        && password.len() >= password_min_length
    {
        Ok(())
    } else {
        return Err(ValidationError::new("PASSWORD_VALIDATION_FAILED"));
    }
}
