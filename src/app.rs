use axum::{Router, Extension};
use sqlx::MySqlPool;

use crate::{routes, config::Config, db};
use crate::clients::geo::GeoApiClient;
use crate::shared::api_clients::ApiClients;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

pub async fn build_app() -> Router {
    let config = Config::from_env();

    let db_pool = db::init_db_pool(&config.database_url).await;

    let geo_client = GeoApiClient::new(config.geo_api_key, config.geo_api_base_url);
    let clients = ApiClients { geo: geo_client };

    let state = AppState { db: db_pool };

    Router::new()
        .merge(routes::create_routes()) // koristi sve rute iz mod.rs
        .layer(Extension(clients))
        .layer(Extension(state))

}
