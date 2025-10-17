mod day1;
use day1::historian_hysteria;
mod day2;
use day2::count_safe_reports;

fn main() {
    println!("Day 1 of Advent of Code (Rust)");
    
    match historian_hysteria() {
        Ok(result) => println!("{}", result),
        Err(err_message) => eprintln!("Error: {}", err_message)
    }

    println!("Day 2 of Advent of Code (Rust)");
    match count_safe_reports(){
        Ok(result) => println!("{result}"),
        Err(err_message) => eprintln!("Error: {}", err_message)
    }
    
    println!("Advent of Code 2024 - Rust Solutions");
}