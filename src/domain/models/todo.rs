use chrono::{ DateTime, Local };
use serde::{ Deserialize, Serialize };
use surrealdb::sql::Thing;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub is_completed: Option<bool>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
}