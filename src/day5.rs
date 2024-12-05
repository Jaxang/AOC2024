use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let (rules, to_print) = parse_input(filename);
    println!("Star 1: {}", star1(rules.clone(), to_print.clone()));
    println!("Star 2: {}", star2(rules.clone(), to_print.clone()));

    Ok(())
}

fn star1(rules: HashMap<i32, Vec<i32>>, to_print: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for result in to_print {
        if is_correct_order(&result, &rules) {
            sum += result[result.len() / 2];
        }
    }
    sum
}

fn star2(rules: HashMap<i32, Vec<i32>>, to_print: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for result in to_print {
        if is_correct_order(&result, &rules) {
            continue;
        }
        let mut counter = HashMap::<i32, HashSet<i32>>::new();
        let nodes = HashSet::from_iter(result.iter().cloned());
        for &num in &result {
            count_dependencies(num, &nodes, &mut counter, &rules);
        }
        let mut sorted_result = result.clone();
        sorted_result.sort_by_key(|&num| counter.get(&num).unwrap().len());
        sum += sorted_result[sorted_result.len() / 2];
    }
    sum
}

fn is_correct_order(result: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    let mut visited = HashMap::new();
    let mut is_correct = true;
    for (i, value) in result.iter().enumerate() {
        if !rules.contains_key(value) {
            visited.insert(value, i as i32);
            continue;
        }

        for j in rules.get(value).unwrap() {
            if visited.contains_key(j) {
                is_correct = false;
                break;
            }
        }

        if !is_correct {
            break;
        }
        visited.insert(value, i as i32);
    }
    is_correct
}

fn count_dependencies(
    current: i32,
    nodes: &HashSet<i32>,
    counter: &mut HashMap<i32, HashSet<i32>>,
    rules: &HashMap<i32, Vec<i32>>,
) {
    if counter.contains_key(&current) {
        return;
    }

    if !rules.contains_key(&current) {
        counter.insert(current, HashSet::new());
        return;
    }

    let mut count = HashSet::new();
    for &num in rules.get(&current).unwrap() {
        if !nodes.contains(&num) {
            continue;
        }

        count_dependencies(num, nodes, counter, rules);
        count.insert(num);
        count = count.union(counter.get(&num).unwrap()).cloned().collect();
    }
    counter.insert(current, count.clone());
}

fn parse_input(filename: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut process_rules = true;
    let mut rules = HashMap::new();
    let mut to_print = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            process_rules = false;
            continue;
        }

        if process_rules {
            let parts: Vec<&str> = line.split("|").collect();
            let key = parts[0].parse::<i32>().unwrap();
            let value = parts[1].parse::<i32>().unwrap();
            rules.entry(key).or_insert_with(Vec::new).push(value);
        } else {
            let mut row = Vec::new();
            let parts: Vec<&str> = line.split(",").collect();
            for s in parts {
                row.push(s.parse::<i32>().unwrap());
            }
            to_print.push(row);
        }
    }

    (rules, to_print)
}
