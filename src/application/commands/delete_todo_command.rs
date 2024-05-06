use axum::{ extract::Path, response::IntoResponse, http::StatusCode, Json };

use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn delete_todo_command(
    Path(todo_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    // let id = todo_id.to_string();

    if let Ok(_) = repository.get_by_id(todo_id.clone()).await {
        let _ = repository.delete_todo(todo_id.clone()).await.unwrap();

        return Ok(StatusCode::NO_CONTENT);
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Todo with id: {} not found", todo_id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
} 