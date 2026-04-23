use axum::extract::Query;
use axum::Json;

use crate::clients::apple_music::AppleMusicClient;
use crate::config::Config;
use crate::models::api::{MessageResponse, MusicSearchResponse, SearchParams};

pub async fn music_test() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: crate::services::music::test_message().to_string(),
    })
}

pub async fn music_search(Query(query): Query<SearchParams>) -> Json<MusicSearchResponse> {
    let config = Config::from_env();
    let client = AppleMusicClient::new(&config);

    Json(crate::services::music::search_music(&client, &query.value))
}