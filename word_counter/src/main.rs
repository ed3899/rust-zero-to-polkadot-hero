use std::env;
use std::fs;

fn main() {
    // Collect CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: word_counter <file>");
        return;
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .expect("Failed to read file");

    let word_count = content.split_whitespace().count();
    println!("Word count: {word_count}");
}
