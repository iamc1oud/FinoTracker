use std::env;


use mongodb::{Client, Collection};

use crate::models::{auth::account::{Account, MongoAccountRepository}, user::user::{MongoUserRepository, User, UserRepository}};

// Collection
enum AppCollections {
    Users,
    Account
}

impl ToString for AppCollections {
    fn to_string(&self) -> String {
        match self {
            AppCollections::Users => "users".to_string(),
            AppCollections::Account => "accounts".to_string(),
        }
    }
}

pub struct Database {
    pub user: MongoUserRepository,
    pub account: MongoAccountRepository,
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
        let user_repository = MongoUserRepository::new(user);

        let account: Collection<Account> = db.collection(&AppCollections::Account.to_string());
        let account_repository = MongoAccountRepository::new(account);

        return Database { user: user_repository, account: account_repository };
    }
}
