use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    #[serde(rename = "database_url")]
    url: String,
    max_connections: u32,
    min_connections: u32,
    connect_timeout_seconds: u64,
    idle_timeout_seconds: u64,
    max_lifetime_seconds: u64,
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();
        envy::from_env::<PostgresConfig>()
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn min_connections(&self) -> u32 {
        self.min_connections
    }

    pub fn connect_timeout_seconds(&self) -> u64 {
        self.connect_timeout_seconds
    }

    pub fn idle_timeout_seconds(&self) -> u64 {
        self.idle_timeout_seconds
    }

    pub fn max_lifetime_seconds(&self) -> u64 {
        self.max_lifetime_seconds
    }
}
