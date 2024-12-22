use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let (available_patterns, designs) = parse_input(filename);
    println!("Star 1: {}", star1(&available_patterns, &designs));
    println!("Star 2: {}", star2(&available_patterns, &designs));

    Ok(())
}

fn star1(available_patterns: &[String], designs: &[String]) -> i64 {
    count_solvable(available_patterns, designs)
}

fn star2(available_patterns: &[String], designs: &[String]) -> i64 {
    count_all_solutions(available_patterns, designs)
}

fn basic_sub_pattern(design: &str, patterns: &[String], cache: &mut HashMap<String, i64>) -> i64 {
    if design.is_empty() {
        return 1;
    }
    if cache.contains_key(design) {
        return cache[design];
    }
    let mut matched_pattens = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        if design.starts_with(pattern) {
            let subdesign = &design[pattern.len()..design.len()];
            let matches = basic_sub_pattern(subdesign, patterns, cache);
            if matches == 0 {
                continue;
            }
            matched_pattens += matches;
        }
    }
    cache.insert(design.to_string(), matched_pattens);
    matched_pattens
}

fn count_solvable(available_patterns: &[String], designs: &[String]) -> i64 {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for design in designs {
        if basic_sub_pattern(design, available_patterns, &mut cache) > 0 {
            sum += 1;
        }
    }
    sum
}

fn count_all_solutions(available_patterns: &[String], designs: &[String]) -> i64 {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for design in designs {
        if basic_sub_pattern(design, available_patterns, &mut cache) > 0 {
            sum += cache[design];
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
