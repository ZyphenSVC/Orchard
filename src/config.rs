use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(3001);

        Self {
            host,
            port,
        }
    }
}