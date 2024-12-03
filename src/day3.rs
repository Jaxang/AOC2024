use regex::Captures;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let parsed_txt = parse_input(filename)?;
    println!("Star 1: {}", star1(parsed_txt.clone()));
    println!("Star 2: {}", star2(parsed_txt.clone()));

    Ok(())
}

fn star1(parsed_txt: Vec<String>) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    for line in parsed_txt {
        for cap in re.captures_iter(&line) {
            sum += calc_mul(cap);
        }
    }
    sum
}

fn star2(parsed_txt: Vec<String>) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"(mul\(\d+\,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let mut skip = false;
    for line in parsed_txt {
        for cap in re.captures_iter(&line) {
            if &cap[0] == "do()" {
                skip = false;
                continue;
            } else if &cap[0] == "don't()" {
                skip = true;
                continue;
            }

            if skip {
                continue;
            }

            sum += calc_mul(cap);
        }
    }
    sum
}

fn calc_mul(cap: Captures) -> i32 {
    let expression = &cap[0][4..cap[0].len() - 1];
    let parts: Vec<&str> = expression.split(',').collect();
    let a: i32 = parts[0].parse().unwrap();
    let b: i32 = parts[1].parse().unwrap();
    let prod = a * b;
    prod
}

fn parse_input(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        result.push(line);
    }

    Ok(result)
}
