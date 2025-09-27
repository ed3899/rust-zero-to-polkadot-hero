// Import axum's tools to create a web server and handle requests.
use axum::{
    routing::get, // For handling GET requests (like someone asking for your message).
    Router, // The main structure to set up your web server routes.
};
// Import SocketAddr to define the server's address (like where your lemonade stand is).
use std::net::SocketAddr;
// Import TcpListener from tokio to create a connection listener for the server.
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello, Backend Engineer!" // The message you give to visitors.
}

// Tell Tokio to set up the program to run asynchronous tasks.
#[tokio::main]
async fn main() {
    // Create a new Router (like setting up your lemonade stand).
    // Route "/" means when someone visits the main page, call the `hello` function.
    let app = Router::new().route("/", get(hello));

    // Set the address for the server (127.0.0.1:3000, like a street address).
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // Print a message to let you know where the stand is open.
    println!("Listening on {addr}");

    // Create a TCP listener to bind to the address (like setting up a phone line for your stand).
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Start the server using `axum::serve` (open the lemonade stand).
    // `app.into_make_service()` prepares the Router to handle requests.
    // `await` keeps the server running to handle visitors.
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap(); // If something goes wrong, crash the program (simple error handling).
}