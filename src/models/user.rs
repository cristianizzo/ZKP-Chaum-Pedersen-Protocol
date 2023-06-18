use sqlx::{PgPool, Postgres, Error as SqlxError};
use sqlx::pool::PoolConnection;

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub y1: String,
    pub y2: String,
}
