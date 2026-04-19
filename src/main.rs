// fn main() {
//     println!("Hello, world!");
// }

use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    "Backend is running"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://localhost:3001");
    HttpServer::new(|| {
        App::new()
            .service(home)
    })
        .bind("127.0.0.1:3001")?
        .run()
        .await
}