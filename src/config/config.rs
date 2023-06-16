use std::env;
use std::error::Error;

pub struct Config {
    pub server_address: String,
    pub server_port: String,
    pub database_url: String,
    pub rust_log: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        // Load environment variables from .env file, if present
        dotenv::dotenv().ok();

        // Read environment variables and assign to struct fields
        Ok(Config {
            server_address: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("PORT").unwrap_or_else(|_| "5000".to_string()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        })
    }
}
