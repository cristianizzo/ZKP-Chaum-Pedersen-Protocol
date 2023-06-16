use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
