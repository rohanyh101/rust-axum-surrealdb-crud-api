use axum::{ response::IntoResponse, Json, http::StatusCode };

use crate::{
    infrastructure::data::repositories::todo_repository::TodoRepository,
    domain::models::todo::Todo
};

pub async fn get_all_todo_query() -> impl IntoResponse {
    let repository = TodoRepository::new();

    let mut todos: Vec<Todo> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        todos = result;
    }

    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "results": todos.len(),
        "todos": todos
    });

    Json(json_response)
}