use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub refresh_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token: Option<String>,
    pub admin_username: String,
    pub admin_password: String,
}

impl Config {
    pub fn load_or_create() -> anyhow::Result<Self> {
        // Only REFRESH_TOKEN is required.
        let refresh_token = env::var("REFRESH_TOKEN")
            .map_err(|_| anyhow::anyhow!("REFRESH_TOKEN environment variable must be set"))?;

        // Optional.
        let admin_username = env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
        let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "changeme".to_string());

        Ok(Config {
            refresh_token,
            jwt_token: None,
            admin_username,
            admin_password,
        })
    }

    pub fn update_tokens(&mut self, jwt: String, refresh: String) -> anyhow::Result<()> {
        self.jwt_token = Some(jwt);
        self.refresh_token = refresh;
        Ok(())
    }
}
