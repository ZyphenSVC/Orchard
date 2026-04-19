// fn main() {
//     println!("Hello, world!");
// }

use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    "Backend is running"
}

#[get("/heartbeat")]
async fn heartbeat() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

fn api_scope() -> actix_web::Scope {
    web::scope("/api")
    .service(heartbeat)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:3001");
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(api_scope())
    })
        .bind("127.0.0.1:3001")?
        .run()
        .await
}