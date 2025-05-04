use axum::{Router, routing::get, extract::{Query, Extension}, Json};
use serde::Deserialize;
use serde_json::Value;
use crate::services::city_service::search_city_by_name;
use crate::shared::api_clients::ApiClients;


#[derive(Deserialize)]
pub struct CityQuery {
    name: String,
}

pub fn routes() -> Router {
    Router::new().route("/name", get(get_city_by_name))
}

async fn get_city_by_name(
    Query(params): Query<CityQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match search_city_by_name(&clients.geo, &params.name).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch city" })),
    }
}
