use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MailerConfig {
    #[serde(rename = "mailer_host")]
    pub host: String,
    #[serde(rename = "mailer_port")]
    pub port: u16,
    #[serde(rename = "mailer_username")]
    pub username: String,
    #[serde(rename = "mailer_password")]
    pub password: String,
    #[serde(rename = "mailer_from")]
    pub from: String,
    #[serde(rename = "mailer_password_reset_base_url")]
    pub password_reset_base_url: String,
}

impl MailerConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();
        envy::from_env::<MailerConfig>()
    }
}
