use crate::config::Config;
use crate::handlers::auth_handlers::AuthHandler;
use crate::services::zkp_auth;
use log::{error, info};
use std::sync::Arc;
use tonic::transport::Server;

mod config;
mod handlers;
mod services;
mod db;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new()?;

    // Start logging
    env_logger::init();

    // Start connection pool
    let database_url = &config.database_url;
    let db_pool = Arc::new(db::connect(database_url)?);

    let address = format!("{}:{}", config.server_address, config.server_port).parse()?;

    info!("Server listening on {}", address);

    // Start the gRPC server
    Server::builder()
        .add_service(zkp_auth::auth_server::AuthServer::new(AuthHandler::new(db_pool.clone())))
        .serve(address)
        .await?;

    Ok(())
}
