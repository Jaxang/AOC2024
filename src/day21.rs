use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

static NUM_7: Pos = Pos { x: 0, y: 0 };
static NUM_8: Pos = Pos { x: 0, y: 1 };
static NUM_9: Pos = Pos { x: 0, y: 2 };
static NUM_4: Pos = Pos { x: 1, y: 0 };
static NUM_5: Pos = Pos { x: 1, y: 1 };
static NUM_6: Pos = Pos { x: 1, y: 2 };
static NUM_1: Pos = Pos { x: 2, y: 0 };
static NUM_2: Pos = Pos { x: 2, y: 1 };
static NUM_3: Pos = Pos { x: 2, y: 2 };
static NUM_E: Pos = Pos { x: 3, y: 0 };
static NUM_0: Pos = Pos { x: 3, y: 1 };
static NUM_A: Pos = Pos { x: 3, y: 2 };

static DIR_U: Pos = Pos { x: 0, y: 1 };
static DIR_A: Pos = Pos { x: 0, y: 2 };
static DIR_L: Pos = Pos { x: 1, y: 0 };
static DIR_D: Pos = Pos { x: 1, y: 1 };
static DIR_R: Pos = Pos { x: 1, y: 2 };

static NUM_GRID: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['E', '0', 'A'],
];

static DIR_GRID: [[char; 3]; 2] = [['E', 'U', 'A'], ['L', 'D', 'R']];

pub fn run(filename: &str) -> io::Result<()> {
    let line = parse_input(filename);
    println!("Star 1: {}", star1(&line));
    println!("Star 2: {}", star2(&line));

    Ok(())
}

fn star1(text_input: &[Vec<char>]) -> i64 {
    solve(text_input, 2)
}

fn star2(text_input: &[Vec<char>]) -> i64 {
    solve(text_input, 25)
}

fn solve(text_input: &[Vec<char>], intermediet_robots: usize) -> i64 {
    let shortest_paths_all = shortest_paths(intermediet_robots);

    let positions = input_to_positions(text_input);
    let mut sum = 0;
    for (i, code) in positions.iter().enumerate() {
        let s: String = text_input[i][0..3].iter().collect();
        let code_value = s.parse::<i64>().unwrap();

        let cost_1 = shortest_paths_all.get(&(NUM_A, code[0])).unwrap();
        let cost_2 = shortest_paths_all.get(&(code[0], code[1])).unwrap();
        let cost_3 = shortest_paths_all.get(&(code[1], code[2])).unwrap();
        let cost_4 = shortest_paths_all.get(&(code[2], code[3])).unwrap();
        let path_cost = cost_1 + cost_2 + cost_3 + cost_4;

        let complexity = code_value * path_cost;
        sum += complexity;
    }
    sum
}

fn shortest_paths(intermediet_robots: usize) -> HashMap<(Pos, Pos), i64> {
    let mut first_robot_cost = HashMap::new();
    for p in [DIR_A, DIR_D, DIR_L, DIR_R, DIR_U].iter() {
        shortest_path(*p, &DIR_GRID, &mut first_robot_cost);
    }

    let mut previouse_cost = first_robot_cost.clone();
    let mut next_cost = HashMap::new();
    for _ in 0..intermediet_robots - 1 {
        next_cost = HashMap::new();
        for p in [DIR_A, DIR_D, DIR_L, DIR_R, DIR_U].iter() {
            shortest_path_with_cost(*p, &DIR_GRID, &previouse_cost, &mut next_cost);
        }
        previouse_cost = next_cost.clone();
    }

    let mut last_robot_cost = HashMap::new();
    for p in [
        NUM_3, NUM_A, NUM_7, NUM_8, NUM_9, NUM_4, NUM_5, NUM_6, NUM_1, NUM_2, NUM_0,
    ]
    .iter()
    {
        shortest_path_with_cost(*p, &NUM_GRID, &next_cost, &mut last_robot_cost);
    }
    last_robot_cost
}

fn shortest_path(end_pos: Pos, grid: &[[char; 3]], dist: &mut HashMap<(Pos, Pos), i64>) {
    let mut pq = PriorityQueue::new();

    pq.push((end_pos, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, steps), _) = pq.pop().unwrap();
        let (i, j) = (pos.x, pos.y);

        if i < 0 || i >= grid.len() as i64 || j < 0 || j >= grid[0].len() as i64 {
            continue;
        }
        if grid[i as usize][j as usize] == 'E' {
            continue;
        }
        if dist.contains_key(&(pos, end_pos)) {
            continue;
        }
        dist.insert((pos, end_pos), steps + 1); // +1 since we always need to press the action button

        pq.push((Pos { x: i + 1, y: j }, steps + 1), Reverse(steps + 1));
        pq.push((Pos { x: i - 1, y: j }, steps + 1), Reverse(steps + 1));
        pq.push((Pos { x: i, y: j + 1 }, steps + 1), Reverse(steps + 1));
        pq.push((Pos { x: i, y: j - 1 }, steps + 1), Reverse(steps + 1));
    }
}

fn shortest_path_with_cost(
    start: Pos,
    grid: &[[char; 3]],
    cost: &HashMap<(Pos, Pos), i64>,
    dist: &mut HashMap<(Pos, Pos), i64>,
) {
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();

    pq.push((start, DIR_A, false, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, controler_pos, done, steps), _) = pq.pop().unwrap();
        let (i, j) = (pos.x, pos.y);

        if i < 0 || i >= grid.len() as i64 || j < 0 || j >= grid[0].len() as i64 {
            continue;
        }
        if grid[i as usize][j as usize] == 'E' {
            continue;
        }
        if visited.contains(&(start, pos, controler_pos, done)) {
            continue;
        }
        visited.insert((start, pos, controler_pos, done));
        if done {
            dist.entry((start, pos)).or_insert(steps);
            continue;
        } else {
            let push_cost = get_cost(DIR_A, controler_pos, cost) + steps;
            pq.push((pos, DIR_A, true, push_cost), Reverse(push_cost));
        }

        let u_cost = steps + get_cost(DIR_U, controler_pos, cost);
        pq.push(
            (Pos { x: i - 1, y: j }, DIR_U, false, u_cost),
            Reverse(u_cost),
        );

        let d_cost = steps + get_cost(DIR_D, controler_pos, cost);
        pq.push(
            (Pos { x: i + 1, y: j }, DIR_D, false, d_cost),
            Reverse(d_cost),
        );

        let l_cost = steps + get_cost(DIR_L, controler_pos, cost);
        pq.push(
            (Pos { x: i, y: j - 1 }, DIR_L, false, l_cost),
            Reverse(l_cost),
        );

        let r_cost = steps + get_cost(DIR_R, controler_pos, cost);
        pq.push(
            (Pos { x: i, y: j + 1 }, DIR_R, false, r_cost),
            Reverse(r_cost),
        );
    }
}

fn get_cost(action: Pos, controler_pos: Pos, cost: &HashMap<(Pos, Pos), i64>) -> i64 {
    let move_cost = cost.get(&(controler_pos, action)).unwrap();
    *move_cost
}

fn input_to_positions(input: &[Vec<char>]) -> Vec<Vec<Pos>> {
    let mut positions = Vec::new();
    for row in input {
        let mut row_positions = Vec::new();
        for c in row {
            match c {
                '1' => row_positions.push(NUM_1),
                '2' => row_positions.push(NUM_2),
                '3' => row_positions.push(NUM_3),
                '4' => row_positions.push(NUM_4),
                '5' => row_positions.push(NUM_5),
                '6' => row_positions.push(NUM_6),
                '7' => row_positions.push(NUM_7),
                '8' => row_positions.push(NUM_8),
                '9' => row_positions.push(NUM_9),
                '0' => row_positions.push(NUM_0),
                'A' => row_positions.push(NUM_A),
                'E' => row_positions.push(NUM_E),
                _ => (),
            }
        }
        positions.push(row_positions);
    }
    positions
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
