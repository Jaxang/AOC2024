use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn run(filename: &str) -> io::Result<()> {
    let grid = parse_input(filename);
    println!("Star 1: {}", star1(&grid));
    println!("Star 2: {}", star2(&grid));

    Ok(())
}

fn star1(grid: &Vec<Vec<char>>) -> i32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let dims = (height, width);
    let (start, mut rows, mut cols) = parse_grid(grid);
    get_visted_spots(start, dims, &mut rows, &mut cols)
        .unwrap()
        .len() as i32
}

fn star2(grid: &Vec<Vec<char>>) -> i32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let dims = (height, width);
    let (start, mut rows, mut cols) = parse_grid(grid);

    let visited = get_visted_spots(start, dims, &mut rows, &mut cols).unwrap();

    let mut sum = 0;
    for (row, col) in visited.iter() {
        let (row, col) = (*row, *col);

        if (row, col) == start {
            continue;
        }
        let mut rs = rows.clone();
        let mut cs = cols.clone();
        rs.entry(row).or_insert_with(Vec::new).push(col);
        cs.entry(col).or_insert_with(Vec::new).push(row);
        if get_visted_spots(start, dims, &mut rs, &mut cs).is_err() {
            sum += 1;
        }
    }
    // 6, 3
    // 7, 6
    // 7, 7
    // 8, 1
    // 8, 3
    // 9, 7
    sum
}

fn get_visted_spots(
    start: (i32, i32),
    dims: (i32, i32),
    rows: &mut HashMap<i32, Vec<i32>>,
    cols: &mut HashMap<i32, Vec<i32>>,
) -> Result<HashSet<(i32, i32)>, i32> {
    let (height, width) = dims;

    let mut visited = HashSet::new();
    let mut states = HashSet::new();
    let mut direction = Direction::Up;
    let mut current_position = start;
    loop {
        let d = direction as i32;
        let state = (current_position, d);
        if states.contains(&state) {
            return Err(0);
        }
        states.insert(state);

        let row = current_position.0;
        let col = current_position.1;
        match direction {
            Direction::Up => {
                let mut pos = -1;
                if let Some(mut obstacles) = cols.get_mut(&col) {
                    let idx = get_obstacle_idx(row, &mut obstacles, false);
                    if idx > 0 {
                        pos = obstacles[(idx - 1) as usize];
                    }
                }
                for i in pos + 1..row + 1 {
                    visited.insert((i, col));
                }
                if pos == -1 {
                    break;
                }
                current_position = (pos + 1, col);
                direction = Direction::Right;
            }
            Direction::Right => {
                let mut pos = width;
                if let Some(mut obstacles) = rows.get_mut(&row) {
                    let idx = get_obstacle_idx(col, &mut obstacles, false);
                    if idx < obstacles.len() as i32 {
                        pos = obstacles[idx as usize];
                    }
                }
                for i in col..pos {
                    visited.insert((row, i));
                }
                if pos == width {
                    break;
                }
                current_position = (row, pos - 1);
                direction = Direction::Down;
            }
            Direction::Down => {
                let mut pos = height;
                if let Some(mut obstacles) = cols.get_mut(&col) {
                    let idx = get_obstacle_idx(row, &mut obstacles, false);
                    if idx < obstacles.len() as i32 {
                        pos = obstacles[idx as usize];
                    }
                }
                for i in row..pos {
                    visited.insert((i, col));
                }
                if pos == height {
                    break;
                }
                current_position = (pos - 1, col);
                direction = Direction::Left;
            }
            Direction::Left => {
                let mut pos = -1;
                if let Some(mut obstacles) = rows.get_mut(&row) {
                    let idx = get_obstacle_idx(col, &mut obstacles, false);
                    if idx > 0 {
                        pos = obstacles[(idx - 1) as usize];
                    }
                }
                for i in pos + 1..col + 1 {
                    visited.insert((row, i));
                }
                if pos == -1 {
                    break;
                }
                current_position = (row, pos + 1);
                direction = Direction::Up;
            }
        }
    }
    Ok(visited)
}

fn get_obstacle_idx(pos: i32, obstacles: &mut Vec<i32>, sorted: bool) -> i32 {
    if !sorted {
        obstacles.sort();
    }
    let idx = obstacles.partition_point(|&x| x < pos);
    idx as i32
}

fn parse_grid(
    grid: &Vec<Vec<char>>,
) -> ((i32, i32), HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) {
    let mut start = (-1, -1);
    let mut rows = HashMap::new();
    let mut cols = HashMap::new();
    for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if c == '#' {
                rows.entry(row as i32)
                    .or_insert_with(Vec::new)
                    .push(col as i32);
                cols.entry(col as i32)
                    .or_insert_with(Vec::new)
                    .push(row as i32);
            } else if c == '^' {
                start = (row as i32, col as i32);
            }
        }
    }

    (start, rows, cols)
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
