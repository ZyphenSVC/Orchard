use serde::Serialize;
use serde::Deserialize;
 #[derive(Serialize)]
 pub struct MusicSearchResponse {
     pub message: String,
     pub value: String,
     pub source: String,
 }

#[derive(Serialize)]
pub struct HeartBeatResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub value: String,
}