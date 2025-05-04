use std::error::Error;
use crate::clients::geo::GeoApiClient;
use serde_json::Value;

pub async fn search_city_by_name(
    client: &GeoApiClient,
    name: &str,
) -> Result<Value, Box<dyn Error>> {
    client.get_city_by_name(name).await
}

pub async fn get_cities_by_country(
    client: &GeoApiClient,
    country: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_cities_by_country(country).await
}