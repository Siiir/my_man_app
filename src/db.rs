//! Code dependant on database and its system.

use anyhow::Context;
use diesel::{Connection, MysqlConnection};
use std::sync::OnceLock;

pub mod cmd;
pub mod data_types;
pub mod human;
pub mod table_name;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<MysqlConnection>>;

/// Gets the database URL.
///
/// URL can be used to identify connection type & establish respective connections.
pub fn get_url() -> anyhow::Result<&'static str> {
    static DB_URL: OnceLock<String> = OnceLock::new();

    // Lazy get logic.
    if let Some(db_url) = DB_URL.get() {
        // Use safe cache if possible.
        return Ok(db_url);
    } else {
        // Extract value from environment if necessary.
        let db_url = std::env::var(crate::name_of::env_var::db_url!()).context(concat!(
            "Couldn't get the \"",
            crate::name_of::env_var::db_url!(),
            "\" environment variable."
        ))?;
        Ok(&*DB_URL.get_or_init(|| db_url))
    }
}
/// Returns an established connection with database.
pub fn establish_connection() -> anyhow::Result<MysqlConnection> {
    (|| -> anyhow::Result<MysqlConnection> { Ok(MysqlConnection::establish(get_url()?)?) })()
        .context("Failed to establish connection to the database.")
}

pub fn establish_connection_pool_with_context() -> anyhow::Result<DbPool> {
    (|| -> anyhow::Result<_> {
        let database_url = get_url()?;
        let manager = diesel::r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
        Ok(r2d2::Pool::builder().build(manager)?)
    })()
    .context("Failed to create a pool. Database connections have nowhere to swim.")
}
pub fn connect_using_pool_with_context(
    db_pool: &r2d2::Pool<diesel::r2d2::ConnectionManager<MysqlConnection>>,
) -> Result<r2d2::PooledConnection<diesel::r2d2::ConnectionManager<MysqlConnection>>, anyhow::Error>
{
    let connection = db_pool
        .get()
        .context("No free database connection in the pool.")
        .context("Can't connect to database.")?;
    Ok(connection)
}
