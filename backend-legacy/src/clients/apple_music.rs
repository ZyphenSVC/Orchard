use reqwest::Client;

use crate::config::Config;

#[derive(Clone)]
pub struct AppleMusicClient {
    pub base_url: String,
    pub storefront: String,
    pub team_id: String,
    pub key_id: String,
    pub private_key: String,
    pub http: Client,
}

impl AppleMusicClient {
    pub fn new(config: &Config) -> Self {
        Self {
            base_url: config.apple_music_base_url.clone(),
            storefront: config.apple_music_storefront.clone(),
            team_id: config.apple_music_team_id.clone(),
            key_id: config.apple_music_key_id.clone(),
            private_key: config.apple_music_private_key.clone(),
            http: Client::new(),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.team_id.is_empty()
            && !self.key_id.is_empty()
            && !self.private_key.is_empty()
    }
}