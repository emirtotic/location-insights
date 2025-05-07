use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct City {
    pub id: Option<i64>,
    pub name: String,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub population: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

pub async fn find_city_by_name(pool: &MySqlPool, name: &str) -> sqlx::Result<Option<City>> {
    let city = sqlx::query_as_unchecked!(City, "SELECT * FROM cities WHERE name = ?", name)
        .fetch_optional(pool)
        .await?;

    Ok(city)
}

pub async fn insert_city(pool: &MySqlPool, city: &City) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO cities (name, country_code, region, population, latitude, longitude, timezone)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        city.name,
        city.country_code.as_deref(),
        city.region.as_deref(),
        city.population,
        city.latitude,
        city.longitude,
        city.timezone.as_deref()
    )
        .execute(pool)
        .await?;

    Ok(())
}
