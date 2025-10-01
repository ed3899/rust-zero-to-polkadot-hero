// src/handlers.rs
use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::db::DbPool;
use crate::errors::AppError;
use crate::models::{CreateUser, HealthResponse, UpdateUser, User};
use sqlx;
use sqlx::PgPool;

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

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, name, email FROM users WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.db)
    .await?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(AppError::NotFound),
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
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("DB error: {}", err),
        )),
    }
}

/// Update a user by ID
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET name = COALESCE($2, name),
            email = COALESCE($3, email)
        WHERE id = $1
        RETURNING id, name, email
        "#,
        id,
        payload.name,
        payload.email
    )
    .fetch_optional(&state.db)
    .await?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(AppError::NotFound),
    }
}

/// Delete a user by ID
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<String>, AppError> {
    let rows_affected = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
        .execute(&state.db)
        .await?
        .rows_affected();

    if rows_affected == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(Json("User deleted".to_string()))
    }
}
