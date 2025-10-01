mod day1;
use day1::historian_hysteria;

fn main() {
    println!("Day 1 of Advent of Code (Rust)");

    match historian_hysteria() {
        Ok(result) => println!("{}", result),
        Err(err_message) => println!("Error: {}", err_message)
    }
    
    println!("Advent of Code 2024 - Rust Solutions");
}