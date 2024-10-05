use mongodb::{bson::{oid::ObjectId, doc, DateTime}, Collection, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub _id: Option<ObjectId>,

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

    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

pub trait UserRepository {
    async fn instance(&self) -> Collection<User>;

    // Inject the MongoDB collection in the constructor
    fn new(model: Collection<User>) -> Self;

    // Find user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    // Create a new user
    async fn create(&self, user: User) -> Result<(), Error>;
}

// Struct that will implement the UserRepository and inject the MongoDB collection
#[derive(Clone)]
pub struct MongoUserRepository {
    collection: Collection<User>,
}

impl MongoUserRepository {
    pub fn new(collection: Collection<User>) -> Self {
        MongoUserRepository { collection }
    }
}

// Implement the UserRepository trait for MongoUserRepository
impl UserRepository for MongoUserRepository {
    // Injecting the MongoDB collection via the constructor
    fn new(collection: Collection<User>) -> Self {
        MongoUserRepository { collection }
    }

    // Implement the find_by_email method using the MongoDB collection
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let filter = doc! { "email": email };
        let user = self.collection.find_one(filter).await?;
        Ok(user)
    }

    // Implement the create method using the MongoDB collection
    async fn create(&self, user: User) -> Result<(), Error> {
        self.collection.insert_one(user).await?;
        Ok(())
    }

    async fn instance(&self) -> Collection<User> {
        return self.collection.clone();
    }
}
