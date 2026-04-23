use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend_url: String,
    pub apple_music_base_url: String,
    pub apple_music_storefront: String,
    pub apple_music_team_id: String,
    pub apple_music_key_id: String,
    pub apple_music_private_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(3001);
        let frontend_url = std::env::var("FRONTEND_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let apple_music_base_url = env::var("APPLE_MUSIC_BASE_URL")
            .unwrap_or_else(|_| "https://api.music.apple.com".to_string());

        let apple_music_storefront = env::var("APPLE_MUSIC_STOREFRONT").unwrap_or_default();
        let apple_music_team_id = env::var("APPLE_MUSIC_TEAM_ID").unwrap_or_default();
        let apple_music_key_id = env::var("APPLE_MUSIC_KEY_ID").unwrap_or_default();
        let apple_music_private_key = env::var("APPLE_MUSIC_PRIVATE_KEY").unwrap_or_default();


        Self {
            host,
            port,
            frontend_url,
            apple_music_base_url,
            apple_music_storefront,
            apple_music_team_id,
            apple_music_key_id,
            apple_music_private_key,
        }
    }
}