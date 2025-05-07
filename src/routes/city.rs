use crate::app::AppState;
use crate::db::city_repository::{find_city_by_name, insert_city, City};
use crate::services::city_service::{get_cities_by_country, search_city_by_name};
use crate::shared::api_clients::ApiClients;
use crate::shared::errors::AppError;
use axum::http::StatusCode;
use axum::{
    extract::{Extension, Query},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use tracing::{error, info, warn};

#[derive(Deserialize)]
pub struct CityQuery {
    name: String,
}

#[derive(Deserialize)]
pub struct CitiesByCountryQuery {
    country: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/name", get(get_city_by_name))
        .route("/cities", get(handler_cities_by_country))
}

async fn get_city_by_name(
    Query(params): Query<CityQuery>,
    Extension(clients): Extension<ApiClients>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Value>, AppError> {
    let pool = &state.db;

    let data = search_city_by_name(&clients.geo, &params.name)
        .await
        .map_err(|_e| AppError::new("Failed to fetch city", StatusCode::BAD_REQUEST))?;

    if let Some(city_obj) = data.get(0) {
        let country_code = city_obj
            .get("country")
            .and_then(|c| c.get("code"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        if let Some(ref code) = country_code {
            let exists: i64 = sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM countries WHERE code = ?)",
                code
            )
            .fetch_one(pool)
                .await.map_err(|e| AppError::new("Database error", StatusCode::INTERNAL_SERVER_ERROR))?;

            if exists == 0 {
                warn!(
                    "Country with code {} not found. Skipping city insert.",
                    code
                );
                return Ok(Json(data));
            }
        }

        let city = City {
            id: None,
            name: city_obj
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            country_code,
            region: city_obj
                .get("state_or_region")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            population: None,
            latitude: city_obj.get("latitude").and_then(|v| v.as_f64()),
            longitude: city_obj.get("longitude").and_then(|v| v.as_f64()),
            timezone: None,
            created_at: Some(Utc::now()),
        };

        if let Err(err) = insert_city(pool, &city).await {
            error!("Failed to insert city: {:?}", err);
        } else {
            info!("Inserted new city into DB: {:?}", city.name);
        }
    }

    Ok(Json(data))
}

async fn handler_cities_by_country(
    Query(query): Query<CitiesByCountryQuery>,
    Extension(clients): Extension<ApiClients>,
) -> Result<Json<Value>, AppError> {
    let data = get_cities_by_country(&clients.geo, &query.country)
        .await
        .map_err(|_e| {
            AppError::new(
                "Failed to fetch cities for country",
                StatusCode::BAD_REQUEST,
            )
        })?;

    Ok(Json(data))
}
