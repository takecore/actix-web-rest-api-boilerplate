use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn create_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    return r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.");
}
