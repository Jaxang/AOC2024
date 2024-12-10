use std::env;
use std::time::Instant;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];
    let start = Instant::now();
    match day.as_str() {
        "day1" => day1::run("inputs/day1.txt").unwrap(),
        "day2" => day2::run("inputs/day2.txt").unwrap(),
        "day3" => day3::run("inputs/day3.txt").unwrap(),
        "day4" => day4::run("inputs/day4.txt").unwrap(),
        "day5" => day5::run("inputs/day5.txt").unwrap(),
        "day6" => day6::run("inputs/day6.txt").unwrap(),
        "day7" => day7::run("inputs/day7.txt").unwrap(),
        "day8" => day8::run("inputs/day8.txt").unwrap(),
        "day9" => day9::run("inputs/day9.txt").unwrap(),
        "day10" => day10::run("inputs/day10.txt").unwrap(),
        _ => {
            eprintln!("Unknown day: {}", day);
            std::process::exit(1);
        }
    }
    let duration = start.elapsed();
    println!("Total runtime: {:?}", duration);
}
