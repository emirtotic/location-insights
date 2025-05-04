use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod config;
mod db;
mod routes;
mod services;
mod clients;
mod app;
mod shared;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    let app = app::build_app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve::serve(listener, app)
        .await
        .unwrap();
}
