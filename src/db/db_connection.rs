use anyhow::Result;
use log::error;
use sqlx::pool::PoolConnection;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tonic::Status;

pub type DbPool = Pool<Postgres>;

pub async fn connect(database_url: &str, max_connections: u32) -> Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn acquire_db_connection(
    db_pool: &Arc<DbPool>,
) -> Result<PoolConnection<Postgres>, Status> {
    db_pool.acquire().await.map_err(|e| {
        error!("Failed to acquire DB connection: {}", e);
        Status::internal("DB connection internal server error")
    })
}
