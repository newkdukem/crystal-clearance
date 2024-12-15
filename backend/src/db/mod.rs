pub mod models;
pub mod schema;

use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use eyre::Result;

pub fn create_pool() -> Result<Pool<AsyncPgConnection>> {
    let db_url = std::env::var("DATABASE_URL")?;
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool: Pool<AsyncPgConnection> = Pool::builder(config).build()?;
    Ok(pool)
}
