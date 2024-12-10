use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let grid = parse_input(filename);
    println!("Star 1: {}", star1(&grid));
    println!("Star 2: {}", star2(&grid));

    Ok(())
}

fn star1(grid: &Vec<Vec<char>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let antennas_mapping = parse_grid(grid);
    let mut unique_locations = HashSet::new();
    for positions in antennas_mapping.values() {
        for (i, (i_row, i_col)) in positions.iter().enumerate() {
            let (i_row_i32, i_col_i32) = (*i_row as i32, *i_col as i32);
            for (j_row, j_col) in positions[(i + 1)..].iter() {
                let (j_row_i32, j_col_i32) = (*j_row as i32, *j_col as i32);
                let dir = (i_row_i32 - j_row_i32, i_col_i32 - j_col_i32);

                let pos1 = (i_row_i32 + dir.0, i_col_i32 + dir.1);
                let pos2 = (i_row_i32 - 2 * dir.0, i_col_i32 - 2 * dir.1);

                if check_bounds(pos1, height as i32, width as i32) {
                    unique_locations.insert(pos1);
                }
                if check_bounds(pos2, height as i32, width as i32) {
                    unique_locations.insert(pos2);
                }
            }
        }
    }
    unique_locations.len() as i32
}

fn star2(grid: &Vec<Vec<char>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let antennas_mapping = parse_grid(grid);
    let mut unique_locations = HashSet::new();
    for positions in antennas_mapping.values() {
        for (i, (i_row, i_col)) in positions.iter().enumerate() {
            let (i_row_i32, i_col_i32) = (*i_row as i32, *i_col as i32);
            for (j_row, j_col) in positions[(i + 1)..].iter() {
                let (j_row_i32, j_col_i32) = (*j_row as i32, *j_col as i32);
                let dir = (i_row_i32 - j_row_i32, i_col_i32 - j_col_i32);

                let mut k = 0;
                let mut current_position = (i_row_i32 + dir.0 * k, i_col_i32 + dir.1 * k);
                while check_bounds(current_position, height as i32, width as i32) {
                    unique_locations.insert(current_position);
                    k += 1;
                    current_position = (i_row_i32 + dir.0 * k, i_col_i32 + dir.1 * k);
                }

                let mut k = -1;
                let mut current_position = (i_row_i32 + dir.0 * k, i_col_i32 + dir.1 * k);
                while check_bounds(current_position, height as i32, width as i32) {
                    unique_locations.insert(current_position);
                    k -= 1;
                    current_position = (i_row_i32 + dir.0 * k, i_col_i32 + dir.1 * k);
                }
            }
        }
    }
    unique_locations.len() as i32
}

fn check_bounds(postion: (i32, i32), height: i32, width: i32) -> bool {
    postion.0 >= 0 && postion.0 < height && postion.1 >= 0 && postion.1 < width
}

fn parse_grid(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut mapping = HashMap::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if c != '.' {
                mapping.entry(c).or_insert_with(Vec::new).push((row, col));
            }
        }
    }
    mapping
}

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }
    grid
}
