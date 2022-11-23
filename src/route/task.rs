use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::controllers;

pub fn create_route() -> Router {
    Router::new()
        .route("/hello", get(root))
        .route("/tasks", get(controllers::task::all_tasks))
        .route("/task", post(controllers::task::new_task))
        .route("/task/:id", get(controllers::task::task))
        .route("/task/:id", put(controllers::task::update_task))
        .route("/task/:id", delete(controllers::task::delete_task))
}

async fn root() -> &'static str {
    "Hello, World!"
}
