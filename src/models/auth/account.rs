use actix_web::Error;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document, DateTime},
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub _id: Option<ObjectId>,
    pub otp: Option<String>,
    pub device_id: Option<String>,
    pub email: Option<String>,
    pub last_logged_in: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            _id: Some(ObjectId::new()),
            otp: None,
            device_id: None,
            email: None,
            last_logged_in: Some(DateTime::now()),
            created_at: Some(DateTime::now()),
            updated_at: Some(DateTime::now()),
        }
    }
}

pub trait AccountRepository {
    async fn instance(&self) -> Collection<Account>;

    fn new(collection: Collection<Account>) -> Self;
    fn update_at_timestamp(&self, account: Account) -> Account;
    async fn create(&self, email: &str) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct MongoAccountRepository {
    collection: Collection<Account>,
}

impl MongoAccountRepository {
    pub fn new(collection: Collection<Account>) -> Self {
        MongoAccountRepository { collection }
    }
}

impl AccountRepository for MongoAccountRepository {
    async fn instance(&self) -> Collection<Account> {
        return self.collection.clone();
    }

    fn new(collection: Collection<Account>) -> Self {
        MongoAccountRepository { collection }
    }

    fn update_at_timestamp(&self, account: Account) -> Account {
        let mut account = account;
        account.updated_at = Some(DateTime::now());
        return account;
    }

    async fn create(&self, email: &str) -> Result<(), Error> {
        let does_exist = self.collection.find_one(doc! {"email": email}).await;

        if does_exist.is_ok() {
            return Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Account with this email already exists",
            )));
        }

        let _ = self
            .collection
            .insert_one(Account {
                _id: Some(ObjectId::new()),
                created_at: Some(DateTime::now()),
                device_id: None,
                email: Some(email.to_string()),
                last_logged_in: Some(DateTime::now()),
                otp: Some("1234".to_string()),
                updated_at: Some(DateTime::now()),
            })
            .await;

        Ok(())
    }
}
