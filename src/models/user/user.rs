use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: mongodb::bson::oid::ObjectId,

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

    pub profile_picture: Option<String>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl User {
    pub fn new() -> Self {
        User {
            _id: ObjectId::new(),
            first_name: None,
            profile_picture: None,
            last_name: None,
            phone_number: None,
            country_code: None,
            email: None,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}
