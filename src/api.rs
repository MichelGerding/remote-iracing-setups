use crate::models::*;
use reqwest::Client;

pub struct ApexApi {
    client: Client,
}

impl ApexApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> anyhow::Result<RefreshTokenResponse> {
        let req = RefreshTokenRequest {
            refresh_token: refresh_token.to_string(),
        };

        println!("Attempting to refresh JWT token...");

        let response = self.client
            .post("https://auth.apexracinguk.com/auth/refresh-token")
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?;

        let status = response.status();
        println!("Response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Token refresh failed with status {}: {}",
                status,
                error_text
            ));
        }

        let response_text = response.text().await?;

        if response_text.is_empty() {
            return Err(anyhow::anyhow!(
                "Received empty response from token refresh endpoint. This usually means:\n\
                 - The refresh token is invalid or expired\n\
                 - The refresh token format is incorrect\n\
                 Please get a new refresh token from the application"
            ));
        }

        let refresh_response: RefreshTokenResponse = serde_json::from_str(&response_text)
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}. Response was: {}", e, response_text))?;

        Ok(refresh_response)
    }

    pub async fn fetch_metadata(&self) -> anyhow::Result<Metadata> {
        println!("Fetching metadata...");

        let response = self.client
            .get("https://simdata.apexracinguk.com/get-all-metadata")
            .send()
            .await?;

        let status = response.status();
        println!("Metadata response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Metadata fetch failed with status {}: {}",
                status,
                error_text
            ));
        }

        let metadata: Metadata = response.json().await?;
        Ok(metadata)
    }

    pub async fn get_datapack_files(&self, jwt_token: &str) -> anyhow::Result<Vec<DatapackFile>> {
        println!("Fetching datapack files...");

        let response = self.client
            .get("https://member.apexracinguk.com/member/get-datapack-files")
            .header("authorization", jwt_token)
            .send()
            .await?;

        let status = response.status();
        println!("Datapack files response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Failed to get datapack files with status {}: {}",
                status,
                error_text
            ));
        }

        let files: Vec<DatapackFile> = response.json().await?;
        println!("Found {} files", files.len());
        Ok(files)
    }

    pub async fn download_file(&self, jwt_token: &str, datapack_id: &str, session_id: &str, file_name: &str) -> anyhow::Result<Vec<u8>> {
        let file_url = format!(
            "https://member.apexracinguk.com/member/download-datapack-file/{}/{}/{}",
            datapack_id, session_id, file_name
        );

        let response = self.client
            .get(&file_url)
            .header("authorization", jwt_token)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Failed to download file with status {}: {}",
                status,
                error_text
            ));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
