use sqlx::pool::PoolConnection;
use sqlx::{Error, Row};
use sqlx::{FromRow, Postgres};

#[derive(FromRow)]
pub struct User {
    pub auth_id: String,
    pub y1: String,
    pub y2: String,
}

pub async fn find_user_by_id(
    conn: &mut PoolConnection<Postgres>,
    user_hash: &str,
) -> Result<Option<User>, Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE auth_id = $1")
        .bind(user_hash)
        .fetch_optional(conn)
        .await?;

    Ok(user)
}

pub async fn user_exists(
    conn: &mut PoolConnection<Postgres>,
    user_hash: &str,
) -> Result<bool, Error> {
    let user_is_registered: bool =
        sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE auth_id = $1)")
            .bind(user_hash)
            .fetch_one(conn)
            .await?
            .get(0);

    Ok(user_is_registered)
}

pub async fn create_user(
    conn: &mut PoolConnection<Postgres>,
    user_hash: &str,
    y1: &str,
    y2: &str,
) -> Result<(), Error> {
    sqlx::query("INSERT INTO users (auth_id, y1, y2) VALUES ($1, $2, $3)")
        .bind(user_hash)
        .bind(y1)
        .bind(y2)
        .execute(conn)
        .await?;

    Ok(())
}
