use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn hello() -> impl IntoResponse {
    Json(json!({
        "message": "hello world"
    }))
}
