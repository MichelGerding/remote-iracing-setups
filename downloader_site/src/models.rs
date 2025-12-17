use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    #[serde(rename = "idToken")]
    pub id_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatapackFile {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "trackId")]
    pub track_id: u32,
    #[serde(rename = "carId")]
    pub car_id: u32,
    #[serde(rename = "datapackId")]
    pub datapack_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarInfo {
    pub id: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "iracingPath", default)]
    pub iracing_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "iracingPath", default)]
    pub iracing_path: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    pub cars: HashMap<String, CarInfo>,
    pub tracks: HashMap<String, TrackInfo>,
}
