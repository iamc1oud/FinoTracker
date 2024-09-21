use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

pub fn load_config() -> Result<Config, std::env::VarError> {
    dotenv().ok();

    Ok(Config { database_url: env::var("DATABASE_URL")? })
}