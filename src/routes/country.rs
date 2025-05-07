use crate::services::country_service::{
    get_countries_by_region,
    get_country_by_calling_code,
    get_country_by_capital,
    get_country_by_code,
    get_country_by_currency,
    get_country_by_language,
    get_country_by_name,
};
use crate::shared::api_clients::ApiClients;
use crate::shared::errors::AppError;
use axum::http::StatusCode;
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
pub struct CallingCodeQuery {
    code: String,
}

#[derive(Deserialize)]
pub struct LanguageQuery {
    code: String,
}

#[derive(Deserialize)]
pub struct RegionQuery {
    region: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/code", get(handler_by_code))
        .route("/name", get(handler_by_name))
        .route("/capital", get(handler_by_capital))
        .route("/currency", get(handler_by_currency))
        .route("/calling_code", get(handler_by_calling_code))
        .route("/lang", get(handler_by_language))
        .route("/region", get(handler_by_region))
}

async fn handler_by_code(
    Query(query): Query<CountryCodeQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_country_by_code(&clients.geo, &query.code)
        .await
        .map_err(|_e| AppError::new("Failed to fetch country", StatusCode::BAD_REQUEST))?;

    Ok(Json(data))
}

async fn handler_by_name(
    Query(query): Query<CountryNameQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_country_by_name(&clients.geo, &query.name)
        .await
        .map_err(|_e| AppError::new("Failed to fetch country by name", StatusCode::BAD_REQUEST))?;

    Ok(Json(data))
}

async fn handler_by_capital(
    Query(query): Query<CapitalCityQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_country_by_capital(&clients.geo, &query.city)
        .await
        .map_err(|_e| {
            AppError::new(
                "Failed to fetch country by capital",
                StatusCode::BAD_REQUEST,
            )
        })?;

    Ok(Json(data))
}

async fn handler_by_currency(
    Query(query): Query<CurrencyQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_country_by_currency(&clients.geo, &query.currency)
        .await
        .map_err(|_e| {
            AppError::new(
                "Failed to fetch country by currency",
                StatusCode::BAD_REQUEST,
            )
        })?;

    Ok(Json(data))
}

async fn handler_by_calling_code(
    Query(query): Query<CallingCodeQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_country_by_calling_code(&clients.geo, &query.code)
        .await
        .map_err(|_e| {
            AppError::new(
                "Failed to fetch country by calling code",
                StatusCode::BAD_REQUEST,
            )
        })?;

    Ok(Json(data))
}

async fn handler_by_language(
    Query(query): Query<LanguageQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_country_by_language(&clients.geo, &query.code)
        .await
        .map_err(|_e| {
            AppError::new(
                "Failed to fetch country by language code",
                StatusCode::BAD_REQUEST,
            )
        })?;

    Ok(Json(data))
}

async fn handler_by_region(
    Query(query): Query<RegionQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_countries_by_region(&clients.geo, &query.region)
        .await
        .map_err(|_e| {
            AppError::new(
                "Failed to fetch countries by region",
                StatusCode::BAD_REQUEST,
            )
        })?;

    Ok(Json(data))
}
