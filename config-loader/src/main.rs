use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    host: String,
    port: u16,
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?; // propagate file read errors
    let config: Config = serde_json::from_str(&data)?; // propagate parse errors
    Ok(config)
}

fn main() {
    match load_config("config.json") {
        Ok(cfg) => println!("Loaded config: {:?}", cfg),
        Err(e) => eprintln!("Error loading config {e}"),
    }
}
