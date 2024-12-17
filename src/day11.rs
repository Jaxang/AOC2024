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

fn star1(line: &[i64]) -> i64 {
    let sum = calculate_stones_bfs(line, 25);
    sum
}

fn star2(line: &[i64]) -> i64 {
    let sum = calculate_stones_bfs(line, 75);
    sum
}
fn calculate_stones_bfs(nums: &[i64], depth: usize) -> i64 {
    let mut nums_counter = HashMap::new();
    for num in nums {
        add_count(&mut nums_counter, *num, 1);
    }

    let mut d = depth;
    while d > 0 {
        let mut new_nums_counter = HashMap::new();
        for (num, &count) in nums_counter.iter() {
            if *num == 0 {
                add_count(&mut new_nums_counter, 1, count);
            } else {
                match split_if_even_length(*num) {
                    Some((left, right)) => {
                        add_count(&mut new_nums_counter, left, count);
                        add_count(&mut new_nums_counter, right, count);
                    }
                    None => {
                        add_count(&mut new_nums_counter, num * 2024, count);
                    }
                }
            }
        }
        nums_counter = new_nums_counter;
        d -= 1;
    }

    let mut sum = 0;
    for (_, &count) in nums_counter.iter() {
        sum += count;
    }
    sum
}

fn add_count(counter: &mut HashMap<i64, i64>, num: i64, count: i64) {
    let old_count = counter.get(&num).unwrap_or(&0);
    counter.insert(num, old_count + count);
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
