use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let parsed_results = parse_input(filename)?;

    let mut summed_safe = 0;
    for result in parsed_results.iter() {
        if is_safe(result) {
            summed_safe += 1;
        }
    }
    println!("Sum of safe results: {}", summed_safe);

    let mut summed_safe15 = 0;
    for result in parsed_results.iter() {
        if is_safe(result) {
            summed_safe15 += 1;
            continue;
        }
        for i in 0..result.len() {
            let mut new_result = result.clone();
            new_result.remove(i);
            if is_safe(&new_result) {
                summed_safe15 += 1;
                break;
            }
        }
    }
    println!("Sum of safe results: {}", summed_safe15);

    Ok(())
}

fn is_safe(result: &Vec<i32>) -> bool {
    let mut safe = true;
    let first_sign = check_diff(result[0], result[1]).unwrap();
    if first_sign == 0 {
        return false;
    }
    for i in 2..result.len() {
        let sign = check_diff(result[i - 1], result[i]).unwrap();
        if sign != first_sign {
            safe = false;
            break;
        }
    }
    safe
}

fn check_diff(left: i32, right: i32) -> io::Result<i32> {
    let diff = left - right;
    if (diff == 0) || (diff < -3) || (diff > 3) {
        return Ok(0);
    } else if diff < 0 {
        return Ok(-1);
    } else {
        return Ok(1);
    }
}

fn parse_input(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let mut numbers = Vec::new();
        for part in parts {
            if let Ok(num) = part.parse::<i32>() {
                numbers.push(num);
            } else {
                eprintln!("Failed to parse number: {}", part);
            }
        }
        result.push(numbers);
    }

    Ok(result)
}
