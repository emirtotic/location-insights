use crate::services::country_service::{get_country_by_code, get_country_by_name};
use crate::shared::api_clients::ApiClients;
use axum::{
    extract::{Extension, Query},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct CountryCodeQuery {
    code: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/code", get(handler_by_code))
        .route("/name", get(handler_by_name))
}

async fn handler_by_code(
    Query(query): Query<CountryCodeQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_country_by_code(&clients.geo, &query.code).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch country" })),
    }
}

#[derive(Deserialize)]
pub struct CountryNameQuery {
    name: String,
}

async fn handler_by_name(
    Query(query): Query<CountryNameQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_country_by_name(&clients.geo, &query.name).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch country by name" })),
    }
}
