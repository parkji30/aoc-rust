use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};


fn file_reader() -> Result<BufReader<File>, std::io::Error> {
    let current_dir = env::current_dir()?;
    let data_path = current_dir.join("data").join("day2.txt");
    let reader = BufReader::new(File::open(data_path)?);
    Ok(reader)
}

pub fn count_safe_reports() -> Result<i32, Error> {
    let mut safe_reports = 0;
    
    for line in file_reader()?.lines(){
        let line_str = line?.clone();
        
        // convert line_str to vector
        let numbers: Vec<i32> = line_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(); 

        if numbers.len() < 2 {
            continue;
        }

        let mut is_safe = true;
        let is_increasing = numbers[0] < numbers[1];

        for i in 0..numbers.len() - 1{
            // Difference and direction
            let diff = numbers[i+1] - numbers[i];
            let abs_diff = diff.abs();

            // if the difference is not valid (must be between 1 and 3 inclusive)
            if !(1..=3).contains(&abs_diff){
                is_safe = false;
                break;
            }

            if is_increasing && diff <= 0{
                is_safe = false;
                break;
            }

            if !is_increasing && diff >= 0{
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe_reports += 1
        }

    }

    Ok(safe_reports)
}