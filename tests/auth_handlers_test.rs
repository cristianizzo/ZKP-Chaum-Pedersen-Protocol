use nillion::{
    db::{connect as db_connect, DbPool},
    errors::AppError,
    handlers::auth_handlers::AuthHandler,
    models::{insert_user, user_exists, User},
    services::zkp_auth::{auth_server::Auth, RegisterRequest},
};
use std::sync::Arc;
use tonic::Request;

#[tokio::test]
async fn test_register_new_user() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables for testing
    let _ = dotenv::from_filename(".env.test");

    // Initialize database connection
    let database_url = std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");
    let max_connections: u32 = std::env::var("MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("Failed to parse MAX_CONNECTIONS");
    let db_pool = Arc::new(db_connect(&database_url, max_connections).await?);

    // Initialize AuthHandler
    let auth_handler = AuthHandler::new(db_pool.clone());

    // Create a RegisterRequest
    let register_request = Request::new(RegisterRequest {
        user: "new_user".to_string(),
        y1: "y1_value".to_string(),
        y2: "y2_value".to_string(),
    });

    // Call register method
    let response = auth_handler.register(register_request).await;

    // Assert that the response is OK
    assert!(response.is_ok());

    // Verify that the user is registered
    let mut pool = db_pool.acquire().await?;
    let user_hash = hash_to_bigint("new_user")?.to_str_radix(16);
    let user_is_registered = user_exists(&mut pool, &user_hash).await?;
    assert!(user_is_registered);

    Ok(())
}
