use sqlx::pool::PoolConnection;
use sqlx::Error;
use sqlx::{FromRow, Postgres};

#[derive(FromRow)]
pub struct Auth {
    pub auth_id: String,
    pub r1: String,
    pub r2: String,
    pub c: String,
}

pub async fn find_auth_by_id(
    conn: &mut PoolConnection<Postgres>,
    auth_id: &str,
) -> Result<Option<Auth>, Error> {
    let auth = sqlx::query_as::<_, Auth>("SELECT * FROM auth WHERE auth_id = $1")
        .bind(auth_id)
        .fetch_optional(conn)
        .await?;

    Ok(auth)
}

pub async fn insert_or_update_auth(
    conn: &mut PoolConnection<Postgres>,
    user_hash: &str,
    r1: &str,
    r2: &str,
    c: &str,
) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO auth (auth_id, r1, r2, c) VALUES ($1, $2, $3, $4)
         ON CONFLICT (auth_id) DO UPDATE SET r1 = $2, r2 = $3, c = $4",
    )
    .bind(user_hash)
    .bind(r1)
    .bind(r2)
    .bind(c)
    .execute(conn)
    .await?;

    Ok(())
}

pub async fn delete_auth_by_id(
    conn: &mut PoolConnection<Postgres>,
    auth_id: &str,
) -> Result<(), Error> {
    sqlx::query("DELETE FROM auth WHERE auth_id = $1")
        .bind(auth_id)
        .execute(conn)
        .await?;

    Ok(())
}
