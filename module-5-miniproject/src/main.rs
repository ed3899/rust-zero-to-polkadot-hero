// Import reqwest to make HTTP requests (like calling the toy store).
use reqwest;
// Import Deserialize from serde to turn JSON data into Rust structures.
use serde::Deserialize;

// Define a Product struct to hold details from the API's fake product data.
// `Deserialize` lets serde convert JSON into this struct.
// `Debug` lets us print the struct for debugging if needed.
#[derive(Debug, Deserialize)]
struct Product {
    id: i32,             // The product's unique ID.
    title: String,       // The product's name (e.g., "iPhone 9").
    description: String, // A short description.
    price: f64,          // The price as a floating-point number (e.g., 549.0).
}

// Define a Response struct to match the API's JSON wrapper.
// The API returns {"products": [...], "total": ..., "skip": ..., "limit": ...}
#[derive(Debug, Deserialize)]
struct Response {
    products: Vec<Product>, // A vector (list) of products.
}

// Tell Tokio to set up the program to run asynchronous tasks.
#[tokio::main]
// The `Result` return type means the program might succeed (Ok) or fail (Err) with an error.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The API URL to fetch a list of fake products.
    // No API key needed—it's completely free and public!
    let api_url = "https://dummyjson.com/products?limit=5"; // Fetch only 5 products for a quick demo.

    // Send a GET request to the API and wait for the response asynchronously.
    // `reqwest::get` is like dialing the toy store.
    // Pass `api_url` directly (not `&api_url`) since it’s a String and `reqwest::get` accepts it.
    let resp: Response = reqwest::get(api_url) // Fixed: removed `&` to avoid `&&str`.
        .await? // Wait for the response to arrive (async magic!).
        .json() // Parse the JSON into our structs.
        .await?; // Wait for the JSON parsing to finish.

    // Print details from the first product in the list.
    // (The API returns a list, so we grab the first one.)
    if let Some(first_product) = resp.products.first() {
        println!(
            "First product: {} - Description: {} - Price: ${}",
            first_product.title, first_product.description, first_product.price
        );
    } else {
        println!("No products found!");
    }

    // Return Ok(()) to say the program finished successfully.
    Ok(())
}
