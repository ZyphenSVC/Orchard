use axum::Json;
use crate::models::api::HeartBeatResponse;

pub async fn heartbeat() -> Json<HeartBeatResponse> {
    Json(HeartBeatResponse {
        status: "ok".to_string(),
    })
}