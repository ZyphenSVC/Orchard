use axum::extract::Query;
use axum::Json;
use crate::models::api::{ MessageResponse, SearchResponse };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchParams {
    pub value: Option<String>,
}

pub async fn music_test() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: crate::services::music::test_message().to_string()
    })
}

pub async fn search(Query(query): Query<SearchParams>) -> Json<SearchResponse> {
    let value = query.value.unwrap_or("".to_string());
    Json(SearchResponse {
        message: crate::services::music::search_message().to_string(),
        value
    })
}