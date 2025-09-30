// use statements are like saying "I'm going to use these tools from my toolbox."
use axum::{Json, Router, routing::get}; // axum: A web framework for Rust. Router = website map. get = handle GET requests.
use dotenvy::dotenv; // dotenvy: Reads variables from a `.env` file (like configuration notes).
use serde::Serialize; // serde: Helps us turn Rust data into JSON (text computers talk in).
use std::env; // env: Lets us read environment variables (like PORT=3000).
use std::net::SocketAddr; // SocketAddr: Represents an IP address + port (like "127.0.0.1:3000").
use tokio::net::TcpListener; // TcpListener: A "door" that waits for people trying to connect.
use tracing_subscriber; // tracing_subscriber: A logging tool (shows messages about what’s happening).

// A struct is like a custom container for data.
// This one holds the "status" we want to send back for the health check.
#[derive(Serialize)] // Tells Rust: "This struct can be safely turned into JSON."
struct HealthResponse {
    status: String,
}

// This function handles requests to the `/health` endpoint.
// It returns `{"status": "ok"}` as JSON to show the server is alive.
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

// The main function. `#[tokio::main]` means it runs in "async mode" (super-efficient multitasking).
#[tokio::main]
async fn main() {
    // Load environment variables from `.env` file if it exists.
    dotenv().ok();

    // Look for the PORT variable in the environment (e.g., PORT=8080).
    // If not found, default to 3000. Then parse it into a number.
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Setup logging so we can see messages in the terminal.
    tracing_subscriber::fmt()
        .with_env_filter("rust_rest_api=debug,tower_http=debug") // Show debug logs for our app and tower_http.
        .init();

    // Create the website routes (like a map).
    let app = Router::new()
        .route("/", get(root)) // When someone visits "/", run the `root` function.
        .route("/health", get(health)); // When someone visits "/health", run the `health` function.

    // Decide where the server should live. Example: 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Print a log message to say: "I’m listening here!"
    tracing::info!("Listening on {addr}");

    // --- Axum v0.7+ way to run the server ---

    // 1. Reserve the address and open the "door" for people to connect.
    let listener = TcpListener::bind(&addr).await.unwrap();

    // 2. Give the "door" and the "map" (app) to Axum and start serving.
    axum::serve(listener, app)
        .await // Wait here until the server is stopped (Ctrl+C).
        .unwrap(); // If something goes really wrong, crash.
}

// A function that runs when someone visits `/`.
// It just returns a text message, not JSON.
async fn root() -> &'static str {
    "Hello, Backend Engineer!" // The response people will see in their browser.
}
