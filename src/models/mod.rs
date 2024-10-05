use user::user::User;

pub mod settings;
pub mod user;
pub mod auth;

pub struct  Repository {
    pub user_repository: User,
}

impl Repository {
    pub async fn new(&self) {
        
    }
}