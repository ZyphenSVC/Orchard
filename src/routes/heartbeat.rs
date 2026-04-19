use actix_web::{get, web, HttpResponse, Responder};

#[get("/heartbeat")]
async fn heartbeat() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

/// # Arguments
///
/// * `cfg`: &mut web::ServiceConfig
///
/// returns: heartbeat route for the API.
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(heartbeat);
}