// src/main.rs
// use statements are like saying "I'm going to use these tools from my toolbox."
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

mod db;
mod handlers;
mod models;
mod routes;
mod errors;

use db::connect;
use handlers::AppState;
use routes::create_router;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter("rust_rest_api=debug,tower_http=debug")
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = connect(&database_url).await;

    let state = AppState { db: pool };
    let app = create_router(state);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("Listening on {addr}");

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
