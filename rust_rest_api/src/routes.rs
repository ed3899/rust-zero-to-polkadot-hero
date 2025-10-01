// src/routes.rs
use axum::{routing::{get, post}, Router};

use crate::handlers::{create_user, health, list_users, root, AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/users", post(create_user).get(list_users))
        .with_state(state)
}
