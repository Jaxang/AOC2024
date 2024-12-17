use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub fn run(filename: &str) -> io::Result<()> {
    let line = parse_input(filename);
    println!("Star 1: {}", star1(&line));
    println!("Star 2: {}", star2(&line));

    Ok(())
}

fn star1(grid: &[Vec<char>]) -> i64 {
    let (start, end) = get_start_and_end(grid);
    let dir = Direction::East;

    let mut pq = PriorityQueue::new();
    let mut visited = HashMap::new();

    let initial_state = (start, dir, 0);
    pq.push(initial_state, Reverse(0));
    while !pq.is_empty() {
        let ((pos, dir, steps), _) = pq.pop().unwrap();

        let (i, j) = pos;
        if i < 0
            || i >= grid.len() as i32
            || j < 0
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] == '#'
        {
            continue;
        }
        if pos == end {
            return steps;
        }
        if visited.contains_key(&(pos, dir)) {
            continue;
        }
        visited.insert((pos, dir), steps);

        let mut new_steps = steps;
        match dir {
            Direction::North => {
                new_steps += 1;
                pq.push(
                    ((i - 1, j), Direction::North, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 1000;
                pq.push(((i, j - 1), Direction::West, new_steps), Reverse(new_steps));
                pq.push(((i, j + 1), Direction::East, new_steps), Reverse(new_steps));
                new_steps += 2000;
                pq.push(
                    ((i + 1, j), Direction::South, new_steps),
                    Reverse(new_steps),
                );
            }
            Direction::East => {
                new_steps += 1;
                pq.push(((i, j + 1), Direction::East, new_steps), Reverse(new_steps));
                new_steps += 1000;
                pq.push(
                    ((i - 1, j), Direction::North, new_steps),
                    Reverse(new_steps),
                );
                pq.push(
                    ((i + 1, j), Direction::South, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 2000;
                pq.push(((i, j - 1), Direction::West, new_steps), Reverse(new_steps));
            }
            Direction::South => {
                new_steps += 1;
                pq.push(
                    ((i + 1, j), Direction::South, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 1000;
                pq.push(((i, j - 1), Direction::West, new_steps), Reverse(new_steps));
                pq.push(((i, j + 1), Direction::East, new_steps), Reverse(new_steps));
                new_steps += 2000;
                pq.push(
                    ((i - 1, j), Direction::North, new_steps),
                    Reverse(new_steps),
                );
            }
            Direction::West => {
                new_steps += 1;
                pq.push(((i, j - 1), Direction::West, new_steps), Reverse(new_steps));
                new_steps += 1000;
                pq.push(
                    ((i - 1, j), Direction::North, new_steps),
                    Reverse(new_steps),
                );
                pq.push(
                    ((i + 1, j), Direction::South, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 2000;
                pq.push(((i, j + 1), Direction::East, new_steps), Reverse(new_steps));
            }
        }
    }
    0
}

fn star2(grid: &[Vec<char>]) -> i64 {
    let (start, end) = get_start_and_end(grid);

    let mut pq = PriorityQueue::new();
    let mut visited_s = HashMap::new();
    let mut visited_p = HashMap::new();

    let (ii, jj) = start;
    let initial_state = (start, Direction::East);
    pq.push(
        ((ii, jj + 1), initial_state, Direction::East, 1),
        Reverse(1),
    );
    pq.push(
        ((ii - 1, jj), initial_state, Direction::North, 1001),
        Reverse(1001),
    );
    pq.push(
        ((ii + 1, jj), initial_state, Direction::South, 1001),
        Reverse(1001),
    );
    pq.push(
        ((ii, jj - 1), initial_state, Direction::West, 2001),
        Reverse(2001),
    );
    let mut best_path = -1;
    while !pq.is_empty() {
        let ((pos, parent, dir, steps), _) = pq.pop().unwrap();

        let (i, j) = pos;
        if best_path > -1 && steps > best_path {
            break;
        }
        if i < 0
            || i >= grid.len() as i32
            || j < 0
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] == '#'
        {
            continue;
        }
        if pos == end && best_path == -1 {
            best_path = steps;
        }
        if visited_s.contains_key(&(pos, dir)) && steps > visited_s[&(pos, dir)] {
            continue;
        }
        visited_s.insert((pos, dir), steps);
        visited_p
            .entry((pos, dir))
            .or_insert_with(Vec::new)
            .push(parent);

        let mut new_steps = steps;
        let curr_state = (pos, dir);
        match dir {
            Direction::North => {
                new_steps += 1;
                pq.push(
                    ((i - 1, j), curr_state, Direction::North, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 1000;
                pq.push(
                    ((i, j - 1), curr_state, Direction::West, new_steps),
                    Reverse(new_steps),
                );
                pq.push(
                    ((i, j + 1), curr_state, Direction::East, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 2000;
                pq.push(
                    ((i + 1, j), curr_state, Direction::South, new_steps),
                    Reverse(new_steps),
                );
            }
            Direction::East => {
                new_steps += 1;
                pq.push(
                    ((i, j + 1), curr_state, Direction::East, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 1000;
                pq.push(
                    ((i - 1, j), curr_state, Direction::North, new_steps),
                    Reverse(new_steps),
                );
                pq.push(
                    ((i + 1, j), curr_state, Direction::South, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 2000;
                pq.push(
                    ((i, j - 1), curr_state, Direction::West, new_steps),
                    Reverse(new_steps),
                );
            }
            Direction::South => {
                new_steps += 1;
                pq.push(
                    ((i + 1, j), curr_state, Direction::South, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 1000;
                pq.push(
                    ((i, j - 1), curr_state, Direction::West, new_steps),
                    Reverse(new_steps),
                );
                pq.push(
                    ((i, j + 1), curr_state, Direction::East, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 2000;
                pq.push(
                    ((i - 1, j), curr_state, Direction::North, new_steps),
                    Reverse(new_steps),
                );
            }
            Direction::West => {
                new_steps += 1;
                pq.push(
                    ((i, j - 1), curr_state, Direction::West, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 1000;
                pq.push(
                    ((i - 1, j), curr_state, Direction::North, new_steps),
                    Reverse(new_steps),
                );
                pq.push(
                    ((i + 1, j), curr_state, Direction::South, new_steps),
                    Reverse(new_steps),
                );
                new_steps += 2000;
                pq.push(
                    ((i, j + 1), curr_state, Direction::East, new_steps),
                    Reverse(new_steps),
                );
            }
        }
    }

    let mut along_best_path = vec![vec![false; grid[0].len()]; grid.len()];
    for dir in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        fill_best_paths((end, dir), &visited_p, &mut along_best_path);
    }

    let mut sum = 0;
    for row in along_best_path.iter() {
        for v in row.iter() {
            if *v {
                sum += 1;
            }
        }
    }
    sum
}

fn fill_best_paths(
    state: ((i32, i32), Direction),
    visited_p: &HashMap<((i32, i32), Direction), Vec<((i32, i32), Direction)>>,
    along_best_path: &mut Vec<Vec<bool>>,
) {
    let (i, j) = state.0;
    along_best_path[i as usize][j as usize] = true;
    if visited_p.contains_key(&state) {
        for parent in visited_p[&state].iter() {
            fill_best_paths(*parent, visited_p, along_best_path);
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
