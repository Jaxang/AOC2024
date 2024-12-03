use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let parsed_numbers = parse_input(filename)?;

    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    for (num1, num2) in parsed_numbers {
        left.push(num1);
        right.push(num2);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for (l, r) in left.iter().zip(&right) {
        sum += (l - r).abs();
    }

    println!("Sum of differences: {}", sum);

    let mut left_to_right_map = HashMap::new();
    for l in left.iter() {
        left_to_right_map.insert(l, 0);
    }
    for r in right.iter() {
        if left_to_right_map.contains_key(r) {
            let count = left_to_right_map.get(r).unwrap() + r;

            left_to_right_map.insert(r, count);
        }
    }
    let total_sum: i32 = left_to_right_map.values().sum();
    println!("Total sum of values in map: {}", total_sum);

    Ok(())
}

fn parse_input(filename: &str) -> io::Result<Vec<(i32, i32)>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                result.push((num1, num2));
            } else {
                eprintln!("Failed to parse numbers in line: {}", line);
            }
        } else {
            eprintln!("Invalid line format: {}", line);
        }
    }

    Ok(result)
}
