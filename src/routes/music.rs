use actix_web::{web, get, HttpResponse, Responder};
use crate::models::api::MessageResponse;

#[get("/music/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse {
        message: crate::services::music::test_message().to_string()
    })
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
}