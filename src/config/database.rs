
use std::env;

use actix_web::{middleware::Logger, rt::System};
use mongodb::{Client, Collection};

use crate::models::user::user::User;

// Collection
enum AppCollections {
    Users,
}

impl ToString for AppCollections {
    fn to_string(&self) -> String {
        match self {
            AppCollections::Users => "users".to_string(),
        }
    }
}

pub struct Database {
    pub user: Collection<User>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(value) => value.to_string(),
            Err(_) => {
                std::process::exit(1);
            }
        };

        let client = Client::with_uri_str(uri).await.unwrap_or_else(|e| {
            println!("Error connecting to database: {}", e);
            std::process::exit(1);
        });

        let db = client.database("fino_db");

        let user : Collection<User> = db.collection(&AppCollections::Users.to_string());

        return Database {
            user,
        }
    }
}