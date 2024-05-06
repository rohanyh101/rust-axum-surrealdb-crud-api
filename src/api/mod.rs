use axum::{ response::IntoResponse, Json };

pub mod router;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "I'm alive!, 💚";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}