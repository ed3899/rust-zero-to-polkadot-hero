// src/models.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

/// Payload to update a user
#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}
