use actix_web::{web, get, HttpResponse, Responder};
use actix_web::web::Query;
use crate::models::api::{ MessageResponse, SearchResponse };

#[get("/music/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse {
        message: crate::services::music::test_message().to_string()
    })
}

#[get("/music/search")]
async fn search(query: Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let value = query.get("value").cloned().unwrap_or_default();
    HttpResponse::Ok().json(SearchResponse {
        message: crate::services::music::search_message().to_string(),
        value
    })
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(test);
    cfg.service(search);
}