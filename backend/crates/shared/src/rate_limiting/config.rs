use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RateLimitingConfig {
    #[serde(rename = "rate_limiting_requests")]
    pub requests_per_second: u64,
    #[serde(rename = "rate_limiting_burst_size")]
    pub burst_size: u32,
}

impl RateLimitingConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();
        envy::from_env::<RateLimitingConfig>()
    }
}
