use std::env;

use actix_web::{middleware::Logger, rt::System};
use mongodb::{Client, Collection};

<<<<<<< HEAD
use crate::models::auth::accounts::Accounts;
=======
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
>>>>>>> f931205736a878ab0d7c5226223e1408b218af37

pub struct Database {
    pub user: Collection<User>,
}

impl Database {
    pub async fn init() -> Self {
        let key = "MONGO_URI";

        dotenv::dotenv().ok();

        let uri = match env::var("MONGO_URI") {
            Ok(value) => {
                println!("Loaded MONGO_URI from .env file: {}", value);
                value.to_string()
            }
            Err(e) => {
                println!("couldn't interpret {key}: {e}");
                std::process::exit(1);
            }
        };

        let client = Client::with_uri_str(uri).await.unwrap_or_else(|e| {
            println!("Error connecting to database: {}", e);
            std::process::exit(1);
        });

        let db = client.database("fino_db");

        let user: Collection<User> = db.collection(&AppCollections::Users.to_string());

        return Database { user };
    }
}
