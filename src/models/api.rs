use serde::Serialize;

#[derive(Serialize)]
pub struct HeartBeatResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub message: String,
    pub value: String,
}