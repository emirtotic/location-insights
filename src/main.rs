use std::env;
use dotenvy::dotenv;
use std::net::SocketAddr;
use sqlx::migrate::Migrator;
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

static MIGRATOR: Migrator = sqlx::migrate!();

mod config;
mod db;
mod routes;
mod services;
mod clients;
mod app;
mod shared;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPool::connect(&database_url).await?;

    MIGRATOR.run(&pool).await?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = config::Config::from_env();
    let app = app::build_app(pool.clone()).await;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    info!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
