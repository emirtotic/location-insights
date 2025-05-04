use std::env;

pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub geo_api_key: String,
    pub geo_api_base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let geo_api_key = env::var("GEO_API_KEY").expect("GEO_API_KEY is missing");
        let geo_api_base_url = env::var("GEO_API_BASE_URL").expect("GEO_API_BASE_URL is missing");

        Self {
            port,
            database_url,
            geo_api_key,
            geo_api_base_url,
        }
    }
}
