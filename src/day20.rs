use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
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
    let mut start_dist = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut end_dist = vec![vec![-1; grid[0].len()]; grid.len()];
    shortest_path(end, &grid, &mut end_dist);
    shortest_path(start, &grid, &mut start_dist);
    let start_target = start_dist[end.0 as usize][end.1 as usize];
    let end_target = end_dist[start.0 as usize][start.1 as usize];
    println!("{}, {}", start_target, end_target);
    let mut best_cheats = cheated_paths(
        end_target as i32 - 49,
        1,
        start,
        end,
        grid,
        &start_dist,
        &end_dist,
    );
    best_cheats.sort();
    println!("{:?}", best_cheats);
    best_cheats.len() as i32
}

fn star2(grid: &[Vec<char>]) -> i32 {
    let (start, end) = get_start_and_end(grid);
    let mut start_dist = vec![vec![-1; grid[0].len()]; grid.len()];
    let mut end_dist = vec![vec![-1; grid[0].len()]; grid.len()];
    shortest_path(end, &grid, &mut end_dist);
    shortest_path(start, &grid, &mut start_dist);
    let start_target = start_dist[end.0 as usize][end.1 as usize];
    let end_target = end_dist[start.0 as usize][start.1 as usize];
    println!("{}, {}", start_target, end_target);
    let best_cheats = cheated_paths(
        end_target as i32 - 49,
        19,
        start,
        end,
        grid,
        &start_dist,
        &end_dist,
    );
    let mut occurrences = HashMap::new();
    for cheat in &best_cheats {
        *occurrences.entry(cheat).or_insert(0) += 1;
    }
    let mut sorted_occurrences: Vec<_> = occurrences.iter().collect();
    sorted_occurrences.sort_by(|a, b| Reverse(b.0).cmp(&Reverse(a.0)));
    for (cheat, count) in sorted_occurrences {
        println!("{:?}: {}", cheat, count);
    }
    best_cheats.len() as i32
}

fn shortest_path(end_pos: (i32, i32), grid: &[Vec<char>], dist: &mut Vec<Vec<i32>>) {
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

fn cheated_paths(
    target: i32,
    cheat_length: i32,
    start_pos: (i32, i32),
    end_pos: (i32, i32),
    grid: &[Vec<char>],
    start_dist: &Vec<Vec<i32>>,
    end_dist: &Vec<Vec<i32>>,
) -> Vec<i32> {
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut outputs = Vec::new();
    let cheated_positions = ((0, 0), 0, (-1, -1));
    pq.push((start_pos, 0, cheated_positions), Reverse(0));
    while !pq.is_empty() {
        let ((pos, steps, cheated), _) = pq.pop().unwrap();
        let (i, j) = pos;
        let mut new_cheated = cheated.clone();

        if steps > target {
            break;
        }
        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
            continue;
        }
        if grid[i as usize][j as usize] == '#' {
            if cheated.1 < cheat_length {
                new_cheated = (cheated.0, cheated.1 + 1, (-1, -1));
            } else {
                continue;
            }
        }
        if new_cheated.1 == 0 {
            new_cheated = (pos, 0, (-1, -1));
        } else if grid[i as usize][j as usize] != '#' && cheated.2 == (-1, -1) {
            new_cheated = (new_cheated.0, new_cheated.1 + 1, pos);
            let original_steps = start_dist[pos.0 as usize][pos.1 as usize];
            let original_steps_left = end_dist[pos.0 as usize][pos.1 as usize];
            let new_steps = original_steps_left + steps;
            let saved = original_steps_left + original_steps - new_steps;
            if !visited.contains(&(pos, (new_cheated.0, new_cheated.2))) && new_steps < target {
                outputs.push(saved);
            }
            if cheated.1 == cheat_length {
                visited.insert((pos, (new_cheated.0, new_cheated.2)));
            }
        }
        if visited.contains(&(pos, (new_cheated.0, new_cheated.2))) {
            continue;
        }

        visited.insert((pos, (new_cheated.0, new_cheated.2)));
        pq.push(((i + 1, j), steps + 1, new_cheated), Reverse(steps + 1));
        pq.push(((i - 1, j), steps + 1, new_cheated), Reverse(steps + 1));
        pq.push(((i, j + 1), steps + 1, new_cheated), Reverse(steps + 1));
        pq.push(((i, j - 1), steps + 1, new_cheated), Reverse(steps + 1));
    }
    outputs
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
