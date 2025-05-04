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

    pub async fn get_country_by_capital(
        &self,
        city: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/country/capital/{}", self.base_url.trim_end_matches('/'), city);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Country By Capital] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn get_country_by_currency(
        &self,
        currency: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/country/currency/{}", self.base_url.trim_end_matches('/'), currency);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Country By Currency] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn get_cities_by_country(
        &self,
        country: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/country/cities/{}", self.base_url.trim_end_matches('/'), country);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Cities By Country] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn get_country_by_calling_code(
        &self,
        code: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/country/calling_code/{}",
            self.base_url.trim_end_matches('/'),
            code
        );

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Country By Calling Code] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn get_country_by_language(
        &self,
        code: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/country/lang/{}",
            self.base_url.trim_end_matches('/'),
            code
        );

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Country By Language] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }

    pub async fn get_countries_by_region(
        &self,
        region: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/country/region/{}",
            self.base_url.trim_end_matches('/'),
            region
        );

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);

        let res = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        println!("🌐 [Countries By Region] [{}]: {}", status, text);

        let json: Value = serde_json::from_str(&text)?;
        Ok(json)
    }


}
