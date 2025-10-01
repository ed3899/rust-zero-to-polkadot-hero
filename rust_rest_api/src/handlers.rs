// src/handlers.rs
use axum::{extract::State, Json};
use uuid::Uuid;

use crate::db::DbPool;
use crate::models::{CreateUser, HealthResponse, User};
use sqlx;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

pub async fn root() -> &'static str {
    "Hello, Backend Engineer!"
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let id = Uuid::new_v4();
    let res = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) RETURNING id, name, email",
    )
    .bind(id)
    .bind(&payload.name)
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            // basic error mapping; improve in Part 3
            let msg = format!("DB error: {}", err);
            Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg))
        }
    }
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (axum::http::StatusCode, String)> {
    let res = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(&state.db)
        .await;

    match res {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", err))),
    }
}
