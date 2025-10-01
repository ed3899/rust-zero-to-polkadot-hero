// src/db.rs
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn connect(database_url: &str) -> DbPool {
    Pool::<Postgres>::connect(database_url)
        .await
        .expect("Failed to connect to Postgres")
}
/// This will be used later in a major refactor
pub async fn init_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create Postgres connection pool")
}
