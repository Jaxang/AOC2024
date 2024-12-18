use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Output;

pub fn run(filename: &str) -> io::Result<()> {
    let (registers, program) = parse_input(filename);
    println!("Star 1: {}", star1(registers.clone(), &program));
    println!("Star 2: {}", star2(registers.clone(), &program));

    Ok(())
}

fn star1(registers: Vec<i64>, program: &Vec<u8>) -> String {
    let output = run_program(registers, program);
    output.iter().map(|&id| id.to_string() + ",").collect()
}

fn star2(registers: Vec<i64>, program: &Vec<u8>) -> String {
    /*
    2,4,1,5,7,5,1,6,4,2,5,5,0,3,3,0
    bst
    bxl
    cdv
    bxl
    bxc
    out
    adv
    jnz 0

    output
    while a>0
        bst B = A%8(xxx)
        bxl B = B^5(101)
        cdv C = A/2.pow(B)
        bxl B = B^6(..000110)
        bxc B = B^C
        out output.push(B)
        adv A/2.pow(3) => A/8 => 2^47 < A < 2^48

    last iter
    A_0/8<1 => A_0=1..7 => B_1!=0
    B_4=0
    C_1=B_3
    if B>=3 => C_1=0
    */
    let mut estimated_a = 1;
    let mut done = false;
    while !done {
        for a in estimated_a..(estimated_a * 8) {
            let mut reg = registers.clone();
            reg[0] = a;
            let output = run_program(reg, program);
            let subprogram = &program[(program.len() - output.len())..program.len()];
            if output == subprogram {
                if output.len() == program.len() {
                    done = true;
                    estimated_a = a;
                } else {
                    estimated_a = a * 8;
                }
                println!("{}", a);
                break;
            }
        }
    }

    println!("Done");
    let mut reg2 = registers.clone();
    reg2[0] = estimated_a;
    let output = run_program(reg2, program);
    output.iter().map(|&id| id.to_string() + ",").collect()
    // for a in 1..8 {
    //     let mut reg = registers.clone();
    //     reg[0] = a;
    //     println!("{}", run_program(reg, program));
    // }
    // println!("a_0=3");

    // // for a in 3 * 8..4 * 8 {
    // //     let mut reg = registers.clone();
    // //     reg[0] = a;
    // //     println!("a_1={} => {}", a, run_program(reg, program));
    // // }
    // println!("a_1 in [24, 25, 29, 31]");
    // println!("Pick lowest => a_1=24");

    // for a in 24 * 8..4 * 8 {
    //     let mut reg = registers.clone();
    //     reg[0] = a;
    //     println!("a_1={} => {}", a, run_program(reg, program));
    // }
}

fn run_program(mut registers: Vec<i64>, program: &Vec<u8>) -> Vec<u8> {
    let mut pointer = 0;
    let mut output = Vec::new();
    while pointer < program.len() - 1 {
        let operator = program[pointer];
        let literal = program[pointer + 1];
        match operator {
            0 => adv(literal, &mut registers),
            1 => bxl(literal, &mut registers),
            2 => bst(literal, &mut registers),
            3 => {
                if let Some(jump) = jnz(literal, &mut registers) {
                    pointer = jump as usize;
                    continue;
                }
            }
            4 => bxc(literal, &mut registers),
            5 => output.push(out(literal, &mut registers)),
            6 => bdv(literal, &mut registers),
            7 => cdv(literal, &mut registers),
            _ => println!("error2"),
        }
        pointer += 2;
    }
    output
}

fn adv(literal: u8, registers: &mut Vec<i64>) {
    let pow = get_combo(literal, registers).try_into().unwrap();
    let base: i64 = 2;
    let denominator = base.pow(pow);
    registers[0] = registers[0] / denominator;
}

fn bxl(literal: u8, registers: &mut Vec<i64>) {
    registers[1] = registers[1] ^ literal as i64;
}

fn bst(literal: u8, registers: &mut Vec<i64>) {
    let com = get_combo(literal, registers);
    registers[1] = com % 8;
}

fn jnz(literal: u8, registers: &mut Vec<i64>) -> Option<u8> {
    if registers[0] == 0 {
        return None;
    }
    return Some(literal);
}

fn bxc(_literal: u8, registers: &mut Vec<i64>) {
    registers[1] = registers[1] ^ registers[2];
}

fn out(literal: u8, registers: &mut Vec<i64>) -> u8 {
    (get_combo(literal, registers) % 8).try_into().unwrap()
}

fn bdv(literal: u8, registers: &mut Vec<i64>) {
    let pow = get_combo(literal, registers).try_into().unwrap();
    let base: i64 = 2;
    let denominator = base.pow(pow);
    registers[1] = registers[0] / denominator;
}

fn cdv(literal: u8, registers: &mut Vec<i64>) {
    let pow = get_combo(literal, registers).try_into().unwrap();
    let base: i64 = 2;
    let denominator = base.pow(pow);
    registers[2] = registers[0] / denominator;
}

fn get_combo(literal: u8, registers: &mut Vec<i64>) -> i64 {
    if literal < 4 {
        return literal as i64;
    } else if literal == 4 {
        return registers[0];
    } else if literal == 5 {
        return registers[1];
    } else if literal == 6 {
        return registers[2];
    }
    println!("Error");
    -1
}

fn parse_input(filename: &str) -> (Vec<i64>, Vec<u8>) {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let register_a_str = lines.next().unwrap().unwrap();
    let register_a = register_a_str
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse::<i64>()
        .unwrap();

    let register_b_str = lines.next().unwrap().unwrap();
    let register_b = register_b_str
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse::<i64>()
        .unwrap();

    let register_c_str = lines.next().unwrap().unwrap();
    let register_c = register_c_str
        .rsplit_once(' ')
        .unwrap()
        .1
        .parse::<i64>()
        .unwrap();

    let registers = Vec::from([register_a, register_b, register_c]);

    lines.next();

    let program_str = lines.next().unwrap().unwrap();
    let program = program_str
        .rsplit_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    (registers, program)
}
