use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let (available_patterns, designs) = parse_input(filename);
    test_code();
    println!("Star 1: {}", star1(&available_patterns, &designs));
    println!("Star 2: {}", star2(&available_patterns, &designs));

    Ok(())
}

fn test_code() {
    let patterns: Vec<String> = Vec::from(["r", "wr", "b", "g", "bwu", "rb", "gb", "br"])
        .iter()
        .map(|&s| s.to_string())
        .collect();
    let filtered = filter_patterns(&patterns);
    let expected: Vec<String> = Vec::from(["r", "b", "g", "wr", "bwu"])
        .iter()
        .map(|&s| s.to_string())
        .collect();
    assert!(filtered.len() == expected.len());
    for i in expected {
        assert!(filtered.contains(&i));
    }

    let designs = [
        "brwrr".to_string(),
        "bggr".to_string(),
        "gbbr".to_string(),
        "rrbgbr".to_string(),
        "ubwu".to_string(),
        "bwurrg".to_string(),
        "brgr".to_string(),
        "bbrgwb".to_string(),
    ]
    .to_vec();
    assert!(basic_solve(&filtered, &designs) == 6);
}

fn star1(available_patterns: &[String], designs: &[String]) -> i64 {
    basic_solve(available_patterns, designs)
}

fn star2(available_patterns: &[String], designs: &[String]) -> i64 {
    0
}

fn filter_patterns(patterns: &[String]) -> Vec<String> {
    let mut filtered_patterns = Vec::new();
    let mut patterns_sorted = patterns.to_vec();
    patterns_sorted.sort_by_key(|a| a.len());
    for pattern in patterns_sorted {
        if !basic_sub_pattern(&pattern, &filtered_patterns) {
            filtered_patterns.push(pattern);
        }
    }
    filtered_patterns
}

fn basic_sub_pattern(design: &str, available_patterns: &[String]) -> bool {
    if design.len() == 0 {
        return true;
    }
    for pattern in available_patterns.iter() {
        if design.starts_with(pattern) {
            let subdesign = &design[pattern.len()..design.len()];
            if basic_sub_pattern(subdesign, available_patterns) {
                return true;
            }
        }
    }
    false
}

fn basic_solve(available_patterns: &[String], designs: &[String]) -> i64 {
    let filted_patterns = filter_patterns(available_patterns);
    let mut sum = 0;
    for design in designs {
        if basic_sub_pattern(design, &filted_patterns) {
            sum += 1;
        }
    }
    sum
}

fn parse_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut available_patterns = Vec::new();
    for p in lines.next().unwrap().unwrap().split(", ") {
        let pattern = String::from(p);
        available_patterns.push(pattern);
    }
    lines.next();

    let mut designs = Vec::new();
    for line in lines {
        designs.push(line.unwrap());
    }

    (available_patterns, designs)
}
