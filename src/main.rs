use std::env;
use std::io;
use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day16;
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
        "all" => run_all(),
        "day1" => run_day(day1::run, "day1"),
        "day2" => run_day(day2::run, "day2"),
        "day3" => run_day(day3::run, "day3"),
        "day4" => run_day(day4::run, "day4"),
        "day5" => run_day(day5::run, "day5"),
        "day6" => run_day(day6::run, "day6"),
        "day7" => run_day(day7::run, "day7"),
        "day8" => run_day(day8::run, "day8"),
        "day9" => run_day(day9::run, "day9"),
        "day10" => run_day(day10::run, "day10"),
        "day11" => run_day(day11::run, "day11"),
        "day12" => run_day(day12::run, "day12"),
        "day16" => run_day(day16::run, "day16"),
        _ => {
            eprintln!("Unknown day: {}", day);
            std::process::exit(1);
        }
    }
    let duration = start.elapsed();
    println!("Total runtime: {:?}", duration);
}

fn run_all() {
    run_day(day1::run, "day1");
    run_day(day2::run, "day2");
    run_day(day3::run, "day3");
    run_day(day4::run, "day4");
    run_day(day5::run, "day5");
    run_day(day6::run, "day6");
    run_day(day7::run, "day7");
    run_day(day8::run, "day8");
    run_day(day9::run, "day9");
    run_day(day10::run, "day10");
    run_day(day11::run, "day11");
    run_day(day12::run, "day12");
    run_day(day16::run, "day16");
}

fn run_day(func: fn(&str) -> io::Result<()>, day: &str) {
    let filename = format!("inputs/{}.txt", day);
    println!("Running {}", day);
    let start = Instant::now();
    func(&filename).unwrap();
    let duration = start.elapsed();
    println!("Runtime: {:?}", duration);
    println!();
}
