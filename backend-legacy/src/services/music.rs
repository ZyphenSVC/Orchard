use crate::clients::apple_music::AppleMusicClient;
use crate::models::api::MusicSearchResponse;

pub fn test_message() -> &'static str {
    "Orchard service is connected"
}

pub fn search_message() -> &'static str {
    "Search service is connected"
}

pub fn search_music(client: &AppleMusicClient, value: &str) -> MusicSearchResponse {
    if client.is_configured() {
        MusicSearchResponse {
            message: "Apple Music client is configured".to_string(),
            value: value.to_string(),
            source: "apple_music_ready".to_string(),
        }
    } else {
        MusicSearchResponse {
            message: "Apple Music client is not configured yet".to_string(),
            value: value.to_string(),
            source: "placeholder".to_string(),
        }
    }
}