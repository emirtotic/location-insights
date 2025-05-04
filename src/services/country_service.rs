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
