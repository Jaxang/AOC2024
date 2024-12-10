use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let equations = parse_input(filename);
    println!("Star 1: {}", star1(&equations));
    println!("Star 2: {}", star2(&equations));

    Ok(())
}

fn star1(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut sum = 0;
    for (control, values) in equations {
        let agg = values[0];
        if check_equation(agg, control, &values[1..], false) {
            sum += control;
        }
    }
    sum
}

fn star2(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut sum = 0;
    for (control, values) in equations {
        let agg = values[0];
        if check_equation(agg, control, &values[1..], true) {
            sum += control;
        }
    }
    sum
}

fn check_equation(agg: i64, control: &i64, values: &[i64], concat_op: bool) -> bool {
    if values.len() == 0 {
        return &agg == control;
    } else if &agg > control {
        return false;
    }
    check_equation(agg + values[0], control, &values[1..], concat_op)
        || check_equation(agg * values[0], control, &values[1..], concat_op)
        || (concat_op
            && check_equation(
                (agg.to_string() + &values[0].to_string()).parse().unwrap(),
                control,
                &values[1..],
                concat_op,
            ))
}

fn parse_input(filename: &str) -> Vec<(i64, Vec<i64>)> {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut equations = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let control_eq_split: Vec<&str> = line.split(": ").collect();
        let control = control_eq_split[0].parse::<i64>().unwrap();
        let eq: Vec<&str> = control_eq_split[1].split(' ').collect();
        let values = eq.into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
        equations.push((control, values));
    }
    equations
}
