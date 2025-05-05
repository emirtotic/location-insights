use axum::{Router, routing::get, extract::{Query, Extension}, Json};
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use crate::services::city_service::{get_cities_by_country, search_city_by_name};
use crate::shared::api_clients::ApiClients;
use crate::shared::errors::AppError;


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
) -> Result<Json<Value>, AppError> {
    let data = search_city_by_name(&clients.geo, &params.name)
        .await
        .map_err(|e| AppError::new("Failed to fetch city", StatusCode::BAD_REQUEST))?;

    Ok(Json(data))
}

async fn handler_cities_by_country(
    Query(query): Query<CitiesByCountryQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_cities_by_country(&clients.geo, &query.country)
        .await
        .map_err(|e| AppError::new("Failed to fetch cities for country", StatusCode::BAD_REQUEST))?;

    Ok(Json(data))
}