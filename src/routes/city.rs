use axum::{Router, routing::get, extract::{Query, Extension}, Json};
use serde::Deserialize;
use serde_json::Value;
use crate::services::city_service::{get_cities_by_country, search_city_by_name};
use crate::shared::api_clients::ApiClients;


#[derive(Deserialize)]
pub struct CityQuery {
    name: String,
}

#[derive(Deserialize)]
pub struct CitiesByCountryQuery {
    country: String,
}

pub fn routes() -> Router {
    Router::new().route("/name", get(get_city_by_name))
        .route("/cities", get(handler_cities_by_country))
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

async fn handler_cities_by_country(
    Query(query): Query<CitiesByCountryQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_cities_by_country(&clients.geo, &query.country).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch cities for country" })),
    }
}
