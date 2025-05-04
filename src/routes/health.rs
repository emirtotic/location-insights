use axum::{routing::get, Router, Json};
use serde_json::json;

pub fn routes() -> Router {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "Location Insights API is running"
    }))
}