use num_bigint::{BigInt, Sign};
use rand::{distributions::Alphanumeric, Rng};
use std::sync::Arc;
use zkp_app::db::connect as db_connect;
use zkp_app::handlers::auth_handlers::AuthHandler;

// Generates a random alphanumeric string of a specified length.
pub fn generate_random_string(length: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    s
}

// Generates random user credentials including a username, password, and its corresponding BigInt.
pub fn generate_user_credentials() -> (String, String, BigInt) {
    let username = generate_random_string(10);
    let password = generate_random_string(5);
    println!("Your username {}", username);
    println!("Your password {}", password);

    let x = BigInt::from_bytes_le(Sign::Plus, &password.as_bytes());

    println!("Your password equates to the integer x = {}", x);

    (username, password, x)
}

pub async fn setup(
) -> Result<(AuthHandler, Arc<sqlx::Pool<sqlx::Postgres>>), Box<dyn std::error::Error>> {
    // Load environment variables for testing
    let _ = dotenv::from_filename(".env.test");

    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let max_connections: u32 = std::env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("Failed to parse MAX_CONNECTIONS");
    let db_pool = Arc::new(db_connect(&database_url, max_connections).await?);

    // Initialize an AuthHandler instance for handling authentication requests.
    let auth_handler = AuthHandler::new(db_pool.clone());

    Ok((auth_handler, db_pool))
}
