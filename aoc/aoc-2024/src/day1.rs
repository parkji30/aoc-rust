use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};


pub fn historian_hysteria() -> Result<i32, Error> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let data_path = current_dir.join("data").join("day1.txt");
    
    let file = File::open(data_path).expect("Failed to open path");
    let reader = BufReader::new(file);
    
    let mut first_journey: Vec<i32> = Vec::new();
    let mut second_journey: Vec<i32> = Vec::new();

    for line in reader.lines(){
        match line {
            Ok(content) => {
                let parts: Vec<&str> = content.split_whitespace().collect();
                let left: i32 = parts[0].parse().expect("Failed to get left");
                let right: i32 = parts[1].parse().expect("Failed to get right");
                first_journey.push(left);
                second_journey.push(right);                
            },
            Err(err_message) => eprintln!("Error reading line: {}", err_message)
        }
    }

    let mut total_distance: i32 = 0;
    
    first_journey.sort();
    second_journey.sort();

    for (first_value, second_value) in first_journey.iter().zip(second_journey.iter()){
        total_distance += (first_value - second_value).abs();
    }
    Ok(total_distance)
}