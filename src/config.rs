#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub frontend_url: String,
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

        Self {
            host,
            port,
            frontend_url,
        }
    }
}