pub mod health;
pub mod city;
pub mod country;

use axum::Router;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/health", health::routes())
        .nest("/cities", city::routes())
        .nest("/countries", country::routes())
}

