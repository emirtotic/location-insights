use serde_json::Value;
use reqwest::{Client, header::HeaderMap};

#[derive(Clone)]
pub struct GeoApiClient {
    pub client: Client,
    pub api_key: String,
    pub base_url: String,
}

impl GeoApiClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client,
            api_key,
            base_url,
        }
    }

    pub async fn get_city_by_name(&self, city: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/city/name/{}", self.base_url.trim_end_matches('/'), city);

        let mut headers = HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 Geo API [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn get_country_by_code(&self, code: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/country/code/{}", self.base_url.trim_end_matches('/'), code);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Country By Code] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn get_country_by_name(&self, name: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/country/name/{}", self.base_url.trim_end_matches('/'), name);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Country By Name] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }


}
