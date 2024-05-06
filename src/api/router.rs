use axum::{
    routing::{ get, post },
    Router,
};

use crate::application::{ 
    commands::{ create_todo_command::create_todo_command, update_todo_command::update_todo_command, delete_todo_command::delete_todo_command }, 
    queries::{ get_all_todo_query::get_all_todo_query, get_todo_by_id_query::get_todo_by_id_query }
};
 
use super::health_check_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))

        .route(
            "/api/todo", 
            post(create_todo_command)
            .get(get_all_todo_query)
        )

        .route(
            "/api/todo/:todo_id",
            get(get_todo_by_id_query)
            .put(update_todo_command)
            .delete(delete_todo_command)
        )
}