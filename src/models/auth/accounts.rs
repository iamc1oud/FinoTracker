use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Accounts {
    #[serde(default)]
    pub first_name: Option<String>,

    #[serde(default)]
    pub last_name: Option<String>,

    #[serde(default)]
    pub phone_number: Option<String>,

    #[serde(default)]
    pub country_code: Option<String>,

    #[serde(default)]
    pub email: Option<String>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            first_name: None,
            last_name: None,
            phone_number: None,
            country_code: None,
            email: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
