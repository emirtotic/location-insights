use crate::services::country_service::{get_cities_by_country, get_countries_by_region, get_country_by_calling_code, get_country_by_capital, get_country_by_code, get_country_by_currency, get_country_by_language, get_country_by_name};
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

#[derive(Deserialize)]
pub struct CountryNameQuery {
    name: String,
}

#[derive(Deserialize)]
pub struct CapitalCityQuery {
    city: String,
}

#[derive(Deserialize)]
pub struct CurrencyQuery {
    currency: String,
}

#[derive(Deserialize)]
pub struct CitiesByCountryQuery {
    country: String,
}

#[derive(Deserialize)]
pub struct CallingCodeQuery {
    code: String,
}

#[derive(Deserialize)]
pub struct LanguageQuery {
    code: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/code", get(handler_by_code))
        .route("/name", get(handler_by_name))
        .route("/capital", get(handler_by_capital))
        .route("/currency", get(handler_by_currency))
        .route("/cities", get(handler_cities_by_country))
        .route("/calling_code", get(handler_by_calling_code))
        .route("/lang", get(handler_by_language))
        .route("/region", get(handler_by_region))



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

async fn handler_by_name(
    Query(query): Query<CountryNameQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_country_by_name(&clients.geo, &query.name).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch country by name" })),
    }
}

async fn handler_by_capital(
    Query(query): Query<CapitalCityQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_country_by_capital(&clients.geo, &query.city).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch country by capital" })),
    }
}

async fn handler_by_currency(
    Query(query): Query<CurrencyQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_country_by_currency(&clients.geo, &query.currency).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch country by currency" })),
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

async fn handler_by_calling_code(
    Query(query): Query<CallingCodeQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_country_by_calling_code(&clients.geo, &query.code).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch by calling code" })),
    }
}

async fn handler_by_language(
    Query(query): Query<LanguageQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_country_by_language(&clients.geo, &query.code).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch by language code" })),
    }
}

#[derive(Deserialize)]
pub struct RegionQuery {
    region: String,
}

async fn handler_by_region(
    Query(query): Query<RegionQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Json<Value> {
    match get_countries_by_region(&clients.geo, &query.region).await {
        Ok(data) => Json(data),
        Err(_) => Json(serde_json::json!({ "error": "Failed to fetch countries by region" })),
    }
}






