use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let line = parse_input(filename);
    println!("Star 1: {}", star1(&line));
    println!("Star 2: {}", star2(&line));

    Ok(())
}

fn star1(grid: &[Vec<char>]) -> i32 {
    let (start, end) = get_start_and_end(grid);
    let mut end_dist = vec![vec![-1; grid[0].len()]; grid.len()];
    shortest_path(end, grid, &mut end_dist);
    let best_cheats = cheated_path(100, 2, start, grid, &end_dist);
    best_cheats.len() as i32
}

fn star2(grid: &[Vec<char>]) -> i32 {
    let (start, end) = get_start_and_end(grid);
    let mut end_dist = vec![vec![-1; grid[0].len()]; grid.len()];
    shortest_path(end, grid, &mut end_dist);
    let best_cheats = cheated_path(100, 20, start, grid, &end_dist);
    best_cheats.len() as i32
}

fn shortest_path(end_pos: (i32, i32), grid: &[Vec<char>], dist: &mut [Vec<i32>]) {
    let mut pq = PriorityQueue::new();

    pq.push((end_pos, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, steps), _) = pq.pop().unwrap();
        let (i, j) = pos;

        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
            continue;
        }
        if grid[i as usize][j as usize] == '#' {
            continue;
        }
        if dist[i as usize][j as usize] != -1 {
            continue;
        }
        dist[i as usize][j as usize] = steps;

        pq.push(((i + 1, j), steps + 1), Reverse(steps + 1));
        pq.push(((i - 1, j), steps + 1), Reverse(steps + 1));
        pq.push(((i, j + 1), steps + 1), Reverse(steps + 1));
        pq.push(((i, j - 1), steps + 1), Reverse(steps + 1));
    }
}

fn cheated_path(
    target: i32,
    cheat_length: i32,
    start_pos: (i32, i32),
    grid: &[Vec<char>],
    end_dist: &Vec<Vec<i32>>,
) -> Vec<i32> {
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut outputs = Vec::new();
    pq.push((start_pos, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, steps), _) = pq.pop().unwrap();
        let (i, j) = pos;

        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
            continue;
        }
        if grid[i as usize][j as usize] == '#' {
            continue;
        }
        cheat(target, cheat_length, pos, grid, end_dist, &mut outputs);

        pq.push(((i + 1, j), steps + 1), Reverse(steps + 1));
        pq.push(((i - 1, j), steps + 1), Reverse(steps + 1));
        pq.push(((i, j + 1), steps + 1), Reverse(steps + 1));
        pq.push(((i, j - 1), steps + 1), Reverse(steps + 1));
    }
    outputs
}

fn cheat(
    target: i32,
    cheat_length: i32,
    start_pos: (i32, i32),
    grid: &[Vec<char>],
    end_dist: &Vec<Vec<i32>>,
    outputs: &mut Vec<i32>,
) {
    let (oi, oj) = start_pos;
    for i in -cheat_length..cheat_length + 1 {
        let ni = oi + i;
        let i_diff = i.abs();
        let j_max = cheat_length - i_diff;
        for j in -j_max..j_max + 1 {
            let nj = oj + j;
            let j_diff = j.abs();
            let ham_dist = i_diff + j_diff;
            if ni < 0
                || ni >= grid.len() as i32
                || nj < 0
                || nj >= grid[0].len() as i32
                || grid[ni as usize][nj as usize] == '#'
            {
                continue;
            }
            //let original_steps = start_dist[pos.0 as usize][pos.1 as usize];
            let original_steps_left = end_dist[oi as usize][oj as usize];
            let steps_left = end_dist[ni as usize][nj as usize];
            let saved = original_steps_left - steps_left - ham_dist;
            if saved >= target {
                outputs.push(saved);
            }
        }
    }
}

fn get_start_and_end(gird: &[Vec<char>]) -> ((i32, i32), (i32, i32)) {
    let mut start = (-1, -1);
    let mut end = (-1, -1);
    for (i, row) in gird.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (i as i32, j as i32);
            }
            if *col == 'E' {
                end = (i as i32, j as i32);
            }
        }
    }
    (start, end)
}

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let lines = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    lines
}
