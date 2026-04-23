pub mod routes;
pub mod config;
pub mod services;
pub mod models;
pub mod clients;

use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use config::Config;
use axum::http::HeaderValue;

async fn home() -> &'static str {
    "Backend is running"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    println!("Server running on http://{}:{}", config.host, config.port);

    let cors = CorsLayer::new()
        .allow_origin(config.frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/", get(home))
        .route("/api/heartbeat", get(routes::heartbeat::heartbeat))
        .route("/api/music/test", get(routes::music::music_test))
        .route("/api/music/search", get(routes::music::music_search))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let addr = SocketAddr::from((config.host.parse::<std::net::IpAddr>().unwrap(), config.port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}