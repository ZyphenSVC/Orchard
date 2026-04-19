// fn main() {
//     println!("Hello, world!");
// }

mod routes;
mod config;

use actix_cors::Cors;
use actix_web::{web, get, App, HttpServer, Responder};
use crate::routes::heartbeat::heartbeat;

#[get("/")]
async fn home() -> impl Responder {
    "Backend is running"
}

fn api_scope() -> actix_web::Scope {
    web::scope("/api")
    .service(heartbeat)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:3001");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .service(home)
            .service(api_scope())
    })
        .bind("127.0.0.1:3001")?
        .run()
        .await
}