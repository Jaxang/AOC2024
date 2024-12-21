use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
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

static DIR_E: Pos = Pos { x: 0, y: 0 };
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

fn star1(text_input: &[Vec<char>]) -> i32 {
    let shortest_paths_num_pad = shortest_paths_num();
    let shortest_paths_dir_pad = shortest_paths_dir();
    let shortest_paths_all = shortest_paths();
    //println!("Shortest paths num all\n{:?}", shortest_paths_all);

    let positions = input_to_positions(text_input);
    let mut sum = 0;
    for (i, code) in positions.iter().enumerate() {
        let s: String = text_input[i][0..3].iter().collect();
        println!("Code: {}A", s);
        let code_value = s.parse::<i32>().unwrap();
        println!("Code value: {}", code_value);

        let cost_1 = shortest_paths_all.get(&(NUM_A, code[0])).unwrap();
        let cost_2 = shortest_paths_all.get(&(code[0], code[1])).unwrap();
        let cost_3 = shortest_paths_all.get(&(code[1], code[2])).unwrap();
        let cost_4 = shortest_paths_all.get(&(code[2], code[3])).unwrap();
        let path_cost = cost_1 + cost_2 + cost_3 + cost_4;
        println!(
            "Path cost: {} ({}, {}, {}, {})",
            path_cost, cost_1, cost_2, cost_3, cost_4
        );

        let complexity = code_value * path_cost;
        println!("Complexity: {}", complexity);
        sum += complexity;

        println!();
    }
    sum
}

fn star2(grid: &[Vec<char>]) -> i32 {
    0
}

fn shortest_paths() -> HashMap<(Pos, Pos), i32> {
    let mut first_robot_cost = HashMap::new();
    for p in [DIR_A, DIR_D, DIR_L, DIR_R, DIR_U].iter() {
        shortest_path(*p, &DIR_GRID, &mut first_robot_cost);
    }
    println!("First robot");
    print_dir_dict(&first_robot_cost);

    let mut second_robot_cost = HashMap::new();
    for p in [DIR_A, DIR_D, DIR_L, DIR_R, DIR_U].iter() {
        shortest_path_with_cost(*p, &DIR_GRID, &first_robot_cost, &mut second_robot_cost);
    }

    println!("Second robot");
    print_dir_dict(&second_robot_cost);

    let mut third_robot_cost = HashMap::new();
    for p in [
        NUM_3, NUM_A, NUM_7, NUM_8, NUM_9, NUM_4, NUM_5, NUM_6, NUM_1, NUM_2, NUM_0,
    ]
    .iter()
    {
        shortest_path_with_cost(*p, &NUM_GRID, &second_robot_cost, &mut third_robot_cost);
    }

    println!("Third robot");
    print_num_dict(&third_robot_cost);
    third_robot_cost
}

fn shortest_paths_num() -> HashMap<(Pos, Pos), i32> {
    let mut dist = HashMap::new();
    for p in [
        NUM_7, NUM_8, NUM_9, NUM_4, NUM_5, NUM_6, NUM_1, NUM_2, NUM_3, NUM_0, NUM_A,
    ]
    .iter()
    {
        shortest_path(*p, &NUM_GRID, &mut dist);
    }

    println!("Num Grid\n{:?}", NUM_GRID);
    for p in [
        NUM_7, NUM_8, NUM_9, NUM_4, NUM_5, NUM_6, NUM_1, NUM_2, NUM_3, NUM_0, NUM_A,
    ]
    .iter()
    {
        println!(
            "A->{}: {}",
            NUM_GRID[p.x as usize][p.y as usize],
            dist.get(&(NUM_A, *p)).unwrap()
        );
    }
    println!();
    dist
}

fn shortest_paths_dir() -> HashMap<(Pos, Pos), i32> {
    let mut dist = HashMap::new();
    for p in [DIR_A, DIR_D, DIR_L, DIR_R, DIR_U].iter() {
        shortest_path(*p, &DIR_GRID, &mut dist);
    }

    println!("Dir Grid\n{:?}", DIR_GRID);
    for p in [DIR_A, DIR_D, DIR_L, DIR_R, DIR_U].iter() {
        println!(
            "^->{}: {}",
            DIR_GRID[p.x as usize][p.y as usize],
            dist.get(&(DIR_U, *p)).unwrap()
        );
    }
    println!();
    dist
}

fn shortest_path(end_pos: Pos, grid: &[[char; 3]], dist: &mut HashMap<(Pos, Pos), i32>) {
    let mut pq = PriorityQueue::new();

    pq.push((end_pos, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, steps), _) = pq.pop().unwrap();
        let (i, j) = (pos.x, pos.y);

        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
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
    cost: &HashMap<(Pos, Pos), i32>,
    dist: &mut HashMap<(Pos, Pos), i32>,
) {
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();

    pq.push((start, DIR_A, false, 0), Reverse(0));
    while !pq.is_empty() {
        let ((pos, controler_pos, done, steps), _) = pq.pop().unwrap();
        let (i, j) = (pos.x, pos.y);

        if i < 0 || i >= grid.len() as i32 || j < 0 || j >= grid[0].len() as i32 {
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

fn get_cost(action: Pos, controler_pos: Pos, cost: &HashMap<(Pos, Pos), i32>) -> i32 {
    let move_cost = cost.get(&(controler_pos, action)).unwrap();
    *move_cost
}

fn print_dir_dict(dist: &HashMap<(Pos, Pos), i32>) {
    for ((s, e), c) in dist.iter() {
        println!("{}->{}: {}", dir_pos_to_char(*s), dir_pos_to_char(*e), c);
    }
}

fn print_num_dict(dist: &HashMap<(Pos, Pos), i32>) {
    for ((s, e), c) in dist.iter() {
        println!("{}->{}: {}", num_pos_to_char(*s), num_pos_to_char(*e), c);
    }
}

fn dir_pos_to_char(pos: Pos) -> char {
    DIR_GRID[pos.x as usize][pos.y as usize]
}

fn num_pos_to_char(pos: Pos) -> char {
    NUM_GRID[pos.x as usize][pos.y as usize]
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
