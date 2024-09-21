use core::panic;
use std::env;

use mongodb::{Client, Collection};

use crate::models::accounts::Accounts;

pub struct Database {
    pub accounts: Collection<Accounts>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("fino_db");

        let accounts : Collection<Accounts> = db.collection("accounts");

        return Database {
            accounts,
        }
    }
}