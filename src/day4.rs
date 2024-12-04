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
    let rows = parsed_txt.len();
    let cols = parsed_txt[0].len();

    for line in parsed_txt.clone() {
        let count = count_all_xmas(line);
        sum += count;
    }

    for i in 0..cols {
        let mut column = String::new();
        for line in parsed_txt.clone().iter() {
            if let Some(c) = line.chars().nth(i) {
                column.push(c);
            }
        }
        let count = count_all_xmas(column);
        sum += count;
    }

    for i in 0..cols {
        let mut row = 0;
        let mut col = i;

        let mut column = String::new();
        while row < rows && col < cols {
            if let Some(c) = parsed_txt[row].chars().nth(col) {
                column.push(c);
            }
            row += 1;
            col += 1;
        }
        sum += count_all_xmas(column);
    }

    for i in 1..rows {
        let mut row = i;
        let mut col = 0;

        let mut column = String::new();
        while row < rows && col < cols {
            if let Some(c) = parsed_txt[row].chars().nth(col) {
                column.push(c);
            }
            row += 1;
            col += 1;
        }
        sum += count_all_xmas(column);
    }

    for i in 1..cols {
        let mut row = rows as i32 - 1;
        let mut col = i;

        let mut column = String::new();
        while row >= 0 && col < cols {
            if let Some(c) = parsed_txt[row as usize].chars().nth(col) {
                column.push(c);
            }
            row -= 1;
            col += 1;
        }
        sum += count_all_xmas(column);
    }

    for i in 0..rows as i32 {
        let mut row = i;
        let mut col = 0;

        let mut column = String::new();
        while row >= 0 && col < cols {
            if let Some(c) = parsed_txt[row as usize].chars().nth(col) {
                column.push(c);
            }
            row -= 1;
            col += 1;
        }
        sum += count_all_xmas(column);
    }

    sum
}

fn star2(parsed_txt: Vec<String>) -> i32 {
    let mut sum = 0;
    let rows = parsed_txt.len();
    let cols = parsed_txt[0].len();
    for row in 0..rows - 2 {
        for col in 0..cols - 2 {
            if parsed_txt[row + 1].chars().nth(col + 1).unwrap() != 'A' {
                continue;
            }
            let mut column = String::new();
            column.push(parsed_txt[row].chars().nth(col).unwrap());
            column.push(parsed_txt[row].chars().nth(col + 2).unwrap());
            column.push(parsed_txt[row + 2].chars().nth(col + 2).unwrap());
            column.push(parsed_txt[row + 2].chars().nth(col).unwrap());
            if column == "MMSS" || column == "SMMS" || column == "SSMM" || column == "MSSM" {
                sum += 1;
            }
        }
    }

    sum
}

fn count_all_xmas(line: String) -> i32 {
    let re1 = Regex::new(r"(XMAS)").unwrap();
    let re2 = Regex::new(r"(SAMX)").unwrap();
    let count1 = re1.find_iter(&line).count() as i32;
    let count2 = re2.find_iter(&line).count() as i32;
    count1 + count2
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
