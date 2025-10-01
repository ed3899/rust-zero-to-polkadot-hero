// src/routes.rs
use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{
    AppState, create_user, delete_user, get_user, health, list_users, root, update_user,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        // split CREATE vs LIST correctly
        .route("/users", post(create_user).get(list_users))
        // single-user operations
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(state)
}
