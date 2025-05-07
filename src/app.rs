use axum::{Router, Extension};
use sqlx::MySqlPool;

use crate::{routes, config::Config};
use crate::clients::geo::GeoApiClient;
use crate::shared::api_clients::ApiClients;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
}

pub async fn build_app(pool: MySqlPool) -> Router {
    let config = Config::from_env();

    let geo_client = GeoApiClient::new(config.geo_api_key, config.geo_api_base_url);
    let clients = ApiClients { geo: geo_client };

    let state = AppState { db: pool };

    Router::new()
        .merge(routes::create_routes())
        .layer(Extension(clients))
        .layer(Extension(state))
}

