use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::time::Duration;

pub async fn init_db_pool(database_url: &str) -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("❌ Failed to connect to the database")
}

