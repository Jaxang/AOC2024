use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let line = parse_input(filename);
    println!("Star 1: {}", star1(&line));
    println!("Star 2: {}", star2(&line));

    Ok(())
}

fn star1(line: &[i64]) -> i32 {
    println!("{:?}", line);
    let mut sum = 0;
    let mut visited = HashMap::new();
    for num in line.iter() {
        sum += calculate_stones(*num, 25, &mut visited);
    }
    sum as i32
}

fn star2(line: &[i64]) -> i32 {
    let mut sum = 0;
    let mut visited = HashMap::new();
    for num in line.iter() {
        sum += calculate_stones(*num, 75, &mut visited);
    }
    sum as i32
}

fn calculate_stones(num: i64, depth: usize, visited: &mut HashMap<i64, (i64, i64, usize)>) -> u32 {
    if depth == 0 {
        return 1;
    }

    if let Some((new_num_l, new_num_r, depth_diff)) = visited.clone().get(&num) {
        //println!("hej");
        if depth_diff > &depth {
            return 1;
        }
        let left_n = calculate_stones(*new_num_l, depth - depth_diff, visited);
        let right_n = calculate_stones(*new_num_r, depth - depth_diff, visited);
        return left_n + right_n;
    }

    let mut traversed = Vec::new();
    let mut new_num = num;
    let mut new_depth = depth;
    if num == 0 {
        traversed.push((0, new_depth));
        new_num = 1;
        new_depth -= 1;
    }
    while new_depth > 0 {
        match split_if_even_length(new_num) {
            Some((left, right)) => {
                new_depth -= 1;
                for (n, d) in traversed.iter() {
                    visited.insert(*n, (left, right, d - new_depth));
                }
                let left_n = calculate_stones(left, new_depth, visited);
                let right_n = calculate_stones(right, new_depth, visited);
                return left_n + right_n;
            }
            None => {
                new_num *= 2024;
                new_depth -= 1;
            }
        }
    }
    1
}

fn split_if_even_length(num: i64) -> Option<(i64, i64)> {
    let ns = num.to_string();
    let n_chars = ns.len();
    if n_chars % 2 == 0 {
        let (left, right) = ns.split_at(n_chars / 2);
        return Some((left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()));
    }
    None
}

fn parse_input(filename: &str) -> Vec<i64> {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}
