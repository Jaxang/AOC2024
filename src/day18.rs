use core::num;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let line = parse_input(filename);
    println!("Star 1: {}", star1(&line));
    println!("Star 2: {}", star2(&line));

    Ok(())
}

fn star1(positions: &[(usize, usize)]) -> i32 {
    let (height, width) = (71, 71);
    let number_of_bytes = 1024;
    let grid = create_grid(positions, (height, width), number_of_bytes);
    shortest_path((0, 0), (height as i32 - 1, width as i32 - 1), &grid).unwrap()
}

fn star2(positions: &[(usize, usize)]) -> String {
    let (height, width) = (71, 71);
    let mut left = 0;
    let mut right = positions.len();
    while left < right {
        let mid = left + (right - left + 1) / 2;
        let grid = create_grid(positions, (height, width), mid);
        let shortest_path = shortest_path((0, 0), (height as i32 - 1, width as i32 - 1), &grid);
        match shortest_path {
            Some(_) => left = mid,
            None => right = mid - 1,
        }
    }
    let p = positions[left];
    p.0.to_string() + "," + &p.1.to_string()
}

fn create_grid(
    positions: &[(usize, usize)],
    size: (usize, usize),
    number_of_bytes: usize,
) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; size.0]; size.1];
    for (i, j) in positions[0..number_of_bytes].iter() {
        grid[*i][*j] = true;
    }
    grid
}

fn shortest_path(start_pos: (i32, i32), end_pos: (i32, i32), grid: &[Vec<bool>]) -> Option<i32> {
    let mut pq = PriorityQueue::new();
    let mut visited = HashMap::new();

    pq.push((start_pos, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, steps), _) = pq.pop().unwrap();
        let (i, j) = pos;

        if i < 0
            || i >= grid.len() as i32
            || j < 0
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize]
        {
            continue;
        }
        if pos == end_pos {
            return Some(steps);
        }
        if visited.contains_key(&pos) {
            continue;
        }
        visited.insert(pos, steps);

        pq.push(((i + 1, j), steps + 1), Reverse(steps + 1));
        pq.push(((i - 1, j), steps + 1), Reverse(steps + 1));
        pq.push(((i, j + 1), steps + 1), Reverse(steps + 1));
        pq.push(((i, j - 1), steps + 1), Reverse(steps + 1));
    }
    None
}

fn parse_input(filename: &str) -> Vec<(usize, usize)> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let mut output = Vec::new();
    for line in reader.lines() {
        let line_s = line.unwrap();
        let (x, y) = line_s.split_once(',').unwrap();
        output.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    output
}
