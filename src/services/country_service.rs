use crate::clients::geo::GeoApiClient;
use serde_json::Value;

pub async fn get_country_by_code(
    client: &GeoApiClient,
    code: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_country_by_code(code).await
}

pub async fn get_country_by_name(
    client: &GeoApiClient,
    name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_country_by_name(name).await
}

pub async fn get_country_by_capital(
    client: &GeoApiClient,
    city: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_country_by_capital(city).await
}

pub async fn get_country_by_currency(
    client: &GeoApiClient,
    currency: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_country_by_currency(currency).await
}

pub async fn get_cities_by_country(
    client: &GeoApiClient,
    country: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_cities_by_country(country).await
}

pub async fn get_country_by_calling_code(
    client: &GeoApiClient,
    code: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_country_by_calling_code(code).await
}

pub async fn get_country_by_language(
    client: &GeoApiClient,
    lang_code: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_country_by_language(lang_code).await
}

pub async fn get_countries_by_region(
    client: &GeoApiClient,
    region: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    client.get_countries_by_region(region).await
}

