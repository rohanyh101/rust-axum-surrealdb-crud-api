use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub content: String,
    pub is_completed: Option<bool>, 
}