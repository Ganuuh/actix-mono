use diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use crate::config::database::DataBaseConfig;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    dotenv::dotenv().ok();
    let config = DataBaseConfig::from_env();
    let manager = ConnectionManager::<PgConnection>::new(&config.url);

    Pool::builder()
        .max_size(config.max_connections)
        .build(manager)
        .expect("Failed to create pool")
}

pub fn establish_connection() -> PgConnection {
    let config = DataBaseConfig::from_env();
    PgConnection::establish(&config.url).expect("Error connecting to database")
}
