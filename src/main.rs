use env_logger::{Builder, Env};
use log::{error, info};
use zkp_app::{
    config::CONFIG, db::connect as db_connect, handlers::auth_handlers::AuthHandler,
    services::zkp_auth,
};
use std::sync::Arc;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or(&CONFIG.rust_log)).init();

    let db_pool = Arc::new(db_connect(&CONFIG.database_url, CONFIG.max_connections).await?);
    info!("Database connected");

    let address = format!("{}:{}", &CONFIG.server_address, &CONFIG.server_port).parse()?;
    info!("Server listening on {}", address);

    Server::builder()
        .add_service(zkp_auth::auth_server::AuthServer::new(AuthHandler::new(
            db_pool.clone(),
        )))
        .serve(address)
        .await?;

    Ok(())
}
