use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub refresh_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token: Option<String>,
    #[serde(default = "default_admin_username")]
    pub admin_username: String,
    #[serde(default = "default_admin_password")]
    pub admin_password: String,
}

fn default_admin_username() -> String {
    "admin".to_string()
}

fn default_admin_password() -> String {
    "changeme".to_string()
}

impl Config {
    pub fn load_or_create() -> anyhow::Result<Self> {
        let config_path = "config.json";

        if Path::new(config_path).exists() {
            let contents = fs::read_to_string(config_path)?;
            let config: Config = serde_json::from_str(&contents)?;
            println!("Loaded configuration from {}", config_path);

            if config.refresh_token == "PASTE_YOUR_REFRESH_TOKEN_HERE" {
                return Err(anyhow::anyhow!(
                    "Please edit config.json and replace 'PASTE_YOUR_REFRESH_TOKEN_HERE' with your actual refresh token"
                ));
            }

            Ok(config)
        } else {
            println!("Config file not found. Creating default config.json");
            println!("Please edit config.json and add your refresh token!");

            let default_config = Config {
                refresh_token: "PASTE_YOUR_REFRESH_TOKEN_HERE".to_string(),
                jwt_token: None,
                admin_username: "admin".to_string(),
                admin_password: "changeme".to_string(),
            };

            let json = serde_json::to_string_pretty(&default_config)?;
            fs::write(config_path, json)?;

            Err(anyhow::anyhow!(
                "Created default config.json. Please edit it with your refresh token and restart."
            ))
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write("config.json", json)?;
        Ok(())
    }

    pub fn update_tokens(&mut self, jwt: String, refresh: String) -> anyhow::Result<()> {
        self.jwt_token = Some(jwt);
        self.refresh_token = refresh;
        self.save()
    }
}
