use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let (start, grid, moves) = parse_input(filename);
    println!("Star 1: {}", star1(start, &grid, &moves));
    println!("Star 2: {}", star2(start, &grid, &moves));

    Ok(())
}

fn star1(start: (usize, usize), grid: &Vec<Vec<char>>, moves: &Vec<char>) -> i64 {
    let mut cur_pos = start;
    let mut g = grid.clone();
    for m in moves {
        //print_grid(cur_pos, &g);
        //println!("{}", m);
        if let Ok(pos) = try_to_move(cur_pos, *m, &mut g) {
            cur_pos = pos;
        }
    }
    //print_grid(cur_pos, &g);
    eval_grid(&g) as i64
}

fn star2(start: (usize, usize), grid: &Vec<Vec<char>>, moves: &Vec<char>) -> i64 {
    let mut cur_pos = (start.0, start.1 * 2);
    let mut upscaled_grid = upscale_grid(grid);
    for m in moves {
        if let Ok(pos) = try_to_move(cur_pos, *m, &mut upscaled_grid) {
            cur_pos = pos;
        }
    }
    eval_grid(&upscaled_grid) as i64
}

fn eval_grid(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (i, row) in grid.iter().enumerate() {
        //println!("{:?}", row);
        for (j, &c) in row.iter().enumerate() {
            if c == 'O' || c == '[' {
                //println!("{}, {}", i, j);
                sum += 100 * i + j;
            }
        }
    }
    sum
}

fn try_to_move(
    pos: (usize, usize),
    move_type: char,
    grid: &mut Vec<Vec<char>>,
) -> Result<(usize, usize), (usize, usize)> {
    if check_ok_move(pos, move_type, grid) {
        do_move(pos, move_type, grid);
        return Ok(get_nex_pos(pos, move_type));
    }
    Err(pos)
}

fn check_ok_move(pos: (usize, usize), move_type: char, grid: &mut Vec<Vec<char>>) -> bool {
    let new_pos = get_nex_pos(pos, move_type);
    let new_pos_obj = grid[new_pos.0][new_pos.1];
    if new_pos_obj == '#' {
        return false;
    } else if new_pos_obj == '.' {
        return true;
    }
    let res = check_ok_move(new_pos, move_type, grid);
    let is_vertical_move = move_type == '^' || move_type == 'v';
    let is_large_box = new_pos_obj == '[' || new_pos_obj == ']';
    if res && is_vertical_move && is_large_box {
        if new_pos_obj == '[' {
            return check_ok_move((new_pos.0, new_pos.1 + 1), move_type, grid);
        } else {
            return check_ok_move((new_pos.0, new_pos.1 - 1), move_type, grid);
        }
    }
    res
}

fn do_move(pos: (usize, usize), move_type: char, grid: &mut Vec<Vec<char>>) {
    let new_pos = get_nex_pos(pos, move_type);
    let new_pos_obj = grid[new_pos.0][new_pos.1];
    let is_box = new_pos_obj == '[' || new_pos_obj == ']' || new_pos_obj == 'O';
    let is_vertical_move = move_type == '^' || move_type == 'v';
    if is_box {
        do_move(new_pos, move_type, grid);
    }
    if is_vertical_move && new_pos_obj == '[' {
        do_move((new_pos.0, new_pos.1 + 1), move_type, grid);
    } else if is_vertical_move && new_pos_obj == ']' {
        do_move((new_pos.0, new_pos.1 - 1), move_type, grid);
    }
    grid[new_pos.0][new_pos.1] = grid[pos.0][pos.1];
    grid[pos.0][pos.1] = '.';
}

fn get_nex_pos(pos: (usize, usize), move_type: char) -> (usize, usize) {
    if move_type == 'v' {
        return (pos.0 + 1, pos.1);
    } else if move_type == '>' {
        return (pos.0, pos.1 + 1);
    } else if move_type == '^' {
        return (pos.0 - 1, pos.1);
    } else {
        return (pos.0, pos.1 - 1);
    }
}

fn upscale_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = Vec::new();
    for row in grid {
        let mut out_row = Vec::new();
        for &c in row {
            if c == '#' {
                out_row.push('#');
                out_row.push('#');
            } else if c == 'O' {
                out_row.push('[');
                out_row.push(']');
            } else {
                out_row.push('.');
                out_row.push('.');
            }
        }
        output.push(out_row);
    }
    output
}

fn parse_input(filename: &str) -> ((usize, usize), Vec<Vec<char>>, Vec<char>) {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut start = (0, 0);
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut i = 0;
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }

        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                start = (i, j);
                row.push('.');
            } else {
                row.push(c);
            }
        }
        grid.push(row);
        i += 1;
    }

    let mut moves: Vec<char> = Vec::new();
    for line in lines {
        for c in line.unwrap().chars() {
            moves.push(c);
        }
    }

    (start, grid, moves)
}
