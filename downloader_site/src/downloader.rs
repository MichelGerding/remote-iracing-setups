use crate::api::Api;
use crate::config::Config;
use crate::models::*;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Downloader {
    api: Api,
    config: Arc<RwLock<Config>>,
    metadata: Arc<RwLock<Option<Metadata>>>,
}

impl Downloader {
    pub fn new(config: Config) -> Self {
        Self {
            api: Api::new(),
            config: Arc::new(RwLock::new(config)),
            metadata: Arc::new(RwLock::new(None)),
        }
    }

    pub fn get_config(&self) -> Arc<RwLock<Config>> {
        self.config.clone()
    }

    pub async fn refresh_jwt(&self) -> anyhow::Result<()> {
        let config = self.config.read().await;
        let refresh_token = config.refresh_token.clone();
        drop(config);

        let response = self.api.refresh_token(&refresh_token).await?;

        let mut config = self.config.write().await;
        config.update_tokens(response.id_token, response.refresh_token)?;

        println!("JWT token refreshed successfully");
        Ok(())
    }

    pub async fn fetch_metadata(&self) -> anyhow::Result<()> {
        let metadata = self.api.fetch_metadata().await?;
        *self.metadata.write().await = Some(metadata);
        println!("Metadata fetched successfully");
        Ok(())
    }

    pub async fn download_files(&self) -> anyhow::Result<usize> {
        let config = self.config.read().await;
        let jwt = config.jwt_token.as_ref()
            .ok_or_else(|| anyhow::anyhow!("JWT token not available"))?
            .clone();
        drop(config);

        let files = self.api.get_datapack_files(&jwt).await?;

        // Only download .sto files
        let files: Vec<_> = files.into_iter()
            .filter(|f| f.file_name.ends_with(".sto"))
            .collect();

        let metadata_guard = self.metadata.read().await;
        let metadata = metadata_guard.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Metadata not loaded"))?;

        let mut downloaded = 0;

        for file in files {
            let car_name = metadata.cars
                .get(&file.car_id.to_string())
                .map(|c| c.iracing_path.clone().unwrap_or_else(|| sanitize_filename(&c.display_name)))
                .unwrap_or_else(|| format!("car_{}", file.car_id));

            let track_name = metadata.tracks
                .get(&file.track_id.to_string())
                .map(|t| sanitize_filename(&t.display_name))
                .unwrap_or_else(|| format!("track_{}", file.track_id));

            let dir_path = PathBuf::from("setups")
                .join(&car_name)
                .join(&track_name);

            fs::create_dir_all(&dir_path)?;

            let file_path = dir_path.join(&file.display_name);

            if file_path.exists() {
                println!("Skipping existing file: {}", file.display_name);
                continue;
            }

            let bytes = self.api.download_file(
                &jwt,
                &file.datapack_id,
                &file.session_id,
                &file.file_name
            ).await?;

            fs::write(&file_path, bytes)?;

            println!("Downloaded: {} -> {}", file.display_name, file_path.display());
            downloaded += 1;
        }

        Ok(downloaded)
    }

    pub async fn get_current_jwt(&self) -> String {
        self.config.read().await
            .jwt_token.clone()
            .unwrap_or_default()
    }

    pub async fn update_refresh_token(&self, new_token: String) -> anyhow::Result<()> {
        let mut config = self.config.write().await;
        config.refresh_token = new_token;
        // No config persistence by design.
        drop(config);

        self.refresh_jwt().await
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}
