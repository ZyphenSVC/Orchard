mod routes;
mod config;

use actix_cors::Cors;
use actix_web::{web, get, App, HttpServer, Responder};
use crate::config::Config;

#[get("/")]
async fn home() -> impl Responder {
    "Backend is running"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    println!("Server running on http://{}:{}", config.host, config.port);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .service(home)
            .service(web::scope("/api"))
            .configure(routes::heartbeat::init)
    })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await
}