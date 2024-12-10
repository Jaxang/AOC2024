use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let grid = parse_input(filename);
    println!("Star 1: {}", star1(&grid));
    println!("Star 2: {}", star2(&grid));

    Ok(())
}

fn star1(grid: &Vec<Vec<usize>>) -> i32 {
    let trail_heads = find_trailheads(grid);
    let mut sum = 0;
    for trailhead in trail_heads {
        let (i, j) = trailhead;
        let set = get_trail_score(grid, i, j);
        sum += set.len();
    }
    sum as i32
}

fn star2(grid: &Vec<Vec<usize>>) -> i32 {
    let trail_heads = find_trailheads(grid);
    let mut sum = 0;
    for trailhead in trail_heads {
        let (i, j) = trailhead;
        let set = get_trail_rating(grid, i, j);
        sum += set.len();
    }
    sum as i32
}

fn find_trailheads(grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

fn get_trail_score(grid: &Vec<Vec<usize>>, i: usize, j: usize) -> HashSet<(usize, usize)> {
    let (i_i32, j_i32) = (i as i32, j as i32);
    let cur_height = grid[i][j];
    if cur_height == 9 {
        let set = HashSet::from([(i, j)]);
        return set;
    }
    let mut set = HashSet::new();
    for (x, y) in [
        (i_i32, j_i32 - 1),
        (i_i32, j_i32 + 1),
        (i_i32 - 1, j_i32),
        (i_i32 + 1, j_i32),
    ]
    .iter()
    {
        if *x < 0 || *y < 0 || *x >= grid.len() as i32 || *y >= grid[0].len() as i32 {
            continue;
        }
        if grid[*x as usize][*y as usize] == cur_height + 1 {
            let other = get_trail_score(grid, *x as usize, *y as usize);
            set.extend(other);
        }
    }
    set
}

fn get_trail_rating(grid: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let (i_i32, j_i32) = (i as i32, j as i32);
    let cur_height = grid[i][j];
    if cur_height == 9 {
        let set = Vec::from([(i, j)]);
        return set;
    }
    let mut set = Vec::new();
    for (x, y) in [
        (i_i32, j_i32 - 1),
        (i_i32, j_i32 + 1),
        (i_i32 - 1, j_i32),
        (i_i32 + 1, j_i32),
    ]
    .iter()
    {
        if *x < 0 || *y < 0 || *x >= grid.len() as i32 || *y >= grid[0].len() as i32 {
            continue;
        }
        if grid[*x as usize][*y as usize] == cur_height + 1 {
            let other = get_trail_rating(grid, *x as usize, *y as usize);
            set.extend(other);
        }
    }
    set
}

fn parse_input(filename: &str) -> Vec<Vec<usize>> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let row = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(row);
    }
    grid
}
