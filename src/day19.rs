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
    let mut sum = 0;
    let pat_counters: Vec<HashMap<char, i64>> = available_patterns
        .iter()
        .map(|p| get_char_count(p))
        .collect();
    for design in designs {
        let counter = get_char_count(design);
        println!("{}", design);
        println!("{:?}", counter);

        let tmp = HashMap::new();
        let viable_pattern_combos =
            build_possible_pattern_sets(0, &tmp.clone(), &counter, &pat_counters);
        if viable_pattern_combos.is_none() {
            println!("No possible pattern sets");
            continue;
        }
        let pattern_sets = viable_pattern_combos.unwrap();
        println!("{:?}", pattern_sets);
        for (j, pattern_set) in pattern_sets.iter().enumerate() {
            println!("Set {}", j);
            for i in pattern_set {
                print!("{}", available_patterns[*i]);
            }
            println!();
        }
        sum += 1;

        // if is_design_possible(
        //     design,
        //     available_patterns,
        //     &counter,
        //     &pat_counters,
        //     &mut pattern_viability,
        // ) {
        //     sum += 1;
        // }
    }
    sum
}

fn star2(available_patterns: &[String], designs: &[String]) -> i64 {
    0
}

fn build_possible_pattern_sets(
    idx: usize,
    curr_char_count: &HashMap<char, i64>,
    design_char_count: &HashMap<char, i64>,
    pattern_char_count: &[HashMap<char, i64>],
) -> Option<Vec<Vec<usize>>> {
    //println!("{:?}", curr_char_count);
    if curr_char_count == design_char_count {
        return Some(vec![Vec::new(); 1]);
    }
    if pattern_char_count.is_empty() {
        return None;
    }

    let pattern_counter = pattern_char_count.first()?;
    let mut possible_patterns = Vec::new();
    let mut new_char_count = curr_char_count.clone();
    let mut participated = 0;
    while check_if_pattern_viable(design_char_count, &new_char_count) {
        let pattern_sets = build_possible_pattern_sets(
            idx + 1,
            &new_char_count,
            design_char_count,
            &pattern_char_count[1..pattern_char_count.len()],
        );
        if pattern_sets.is_some() && participated > 0 {
            for mut p in pattern_sets.unwrap() {
                for _ in 0..participated {
                    p.push(idx);
                }
                possible_patterns.push(p);
            }
        }
        new_char_count = add_counters(&new_char_count, &pattern_counter);
        participated += 1;
    }
    if possible_patterns.is_empty() {
        return None;
    }
    Some(possible_patterns)
}

fn get_possible_pattern_extentions(
    cur_char_count: &HashMap<char, i64>,
    design_char_count: &HashMap<char, i64>,
    pattern_char_count: &Vec<HashMap<char, i64>>,
) -> Vec<usize> {
    let mut possible_extensions = Vec::new();
    for (i, pattern_counter) in pattern_char_count.iter().enumerate() {
        let mut new_char_count = cur_char_count.clone();
        for (&c, v) in pattern_counter {
            *new_char_count.entry(c).or_insert(0) += v;
        }
        if check_if_pattern_viable(design_char_count, &new_char_count) {
            possible_extensions.push(i);
        }
    }
    possible_extensions
}

fn get_char_count(s: &str) -> HashMap<char, i64> {
    let mut counter = HashMap::new();
    for ch in s.chars() {
        *counter.entry(ch).or_insert(0) += 1;
    }
    counter
}

fn add_counters(
    counter1: &HashMap<char, i64>,
    counter2: &HashMap<char, i64>,
) -> HashMap<char, i64> {
    let mut new_counter = counter1.clone();
    for (ch, count) in counter2 {
        *new_counter.entry(*ch).or_insert(0) += count;
    }
    new_counter
}

fn is_design_possible(
    design: &str,
    available_patterns: &[String],
    design_char_count: &HashMap<char, i64>,
    pattern_char_count: &Vec<HashMap<char, i64>>,
    pattern_viability: &mut Vec<bool>,
) -> bool {
    if design.len() == 0 {
        return true;
    }
    for (i, pattern) in available_patterns.iter().enumerate() {
        if !pattern_viability[i] {
            continue;
        } else if !check_if_pattern_viable(design_char_count, &pattern_char_count[i]) {
            pattern_viability[i] = false;
            continue;
        }
        if design.starts_with(pattern) {
            let subdesign = &design[pattern.len()..design.len()];
            if is_design_possible(
                subdesign,
                available_patterns,
                design_char_count,
                pattern_char_count,
                pattern_viability,
            ) {
                return true;
            }
        }
    }
    false
}

fn check_if_pattern_viable(
    design_char_count: &HashMap<char, i64>,
    pattern_char_count: &HashMap<char, i64>,
) -> bool {
    for (ch, count) in pattern_char_count {
        if design_char_count.get(ch).unwrap_or(&0) < count {
            return false;
        }
    }
    true
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
