use num_bigint::BigUint;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let line = parse_input(filename);
    println!("Star 1: {}", star1(&line));
    println!("Star 2: {}", star2(&line));

    Ok(())
}

fn star1(line: &[u32]) -> String {
    let mut checksum: BigUint = BigUint::from(0u32);
    let mut global_pos = line[0];
    let mut current_freespace_idx = 1u32;
    let mut current_freespace_remaining = line[current_freespace_idx as usize];
    let mut last_file_idx = line.len() as u32 - 1u32;
    let mut last_file_remaining = line[last_file_idx as usize];
    while current_freespace_idx < last_file_idx {
        let min_remaining = std::cmp::min(current_freespace_remaining, last_file_remaining);

        let last_id = last_file_idx / 2;
        let new_global_pos = global_pos + min_remaining;
        for i in global_pos..new_global_pos {
            checksum += BigUint::from(last_id * i);
        }

        current_freespace_remaining -= min_remaining;
        last_file_remaining -= min_remaining;
        global_pos = new_global_pos;

        if current_freespace_remaining == 0 {
            let next_idx = current_freespace_idx + 1;
            let next_id = next_idx / 2;
            let next_remaining = if next_idx < last_file_idx {
                line[next_idx as usize]
            } else {
                last_file_remaining
            };
            let other_new_global_pos = global_pos + next_remaining;
            for i in global_pos..other_new_global_pos {
                checksum += BigUint::from(next_id * i);
            }
            current_freespace_idx += 2;
            current_freespace_remaining = line[current_freespace_idx as usize];
            global_pos = other_new_global_pos;
        }
        if last_file_remaining == 0 {
            last_file_idx -= 2;
            last_file_remaining = line[last_file_idx as usize];
        }
    }
    checksum.to_string()
}

fn star2(line: &[u32]) -> String {
    let mut checksum: BigUint = BigUint::from(0u32);
    let mut idx_lut: Vec<PriorityQueue<usize, Reverse<usize>>> = vec![PriorityQueue::new(); 9];
    let mut acc_pos = Vec::new();

    fill_acc_pos(line, &mut acc_pos);
    fill_queue(line, &acc_pos, &mut idx_lut);

    for i in (0..(line.len() - 2)).step_by(2) {
        let mem_idx = line.len() - 1 - i;
        let mem_id = mem_idx as u32 / 2;
        let mem_size = line[line.len() - 1 - i] as usize;
        let mem_pos = acc_pos[mem_idx];

        let (freespace_size, first_freespace) =
            find_first_freespace(&idx_lut, acc_pos[line.len()] as usize, mem_size);

        if first_freespace > mem_pos as usize {
            for j in mem_pos..mem_pos + mem_size as u32 {
                checksum += BigUint::from(mem_id * j);
            }
        } else {
            for j in first_freespace..first_freespace + mem_size {
                checksum += BigUint::from(mem_id * j as u32);
            }

            pop_freespace(&mut idx_lut, freespace_size);
            if mem_size < freespace_size {
                let freespace_left = freespace_size - mem_size;
                push_freespace(&mut idx_lut, freespace_left, first_freespace + mem_size);
            }
        }
    }

    checksum.to_string()
}

fn fill_acc_pos(line: &[u32], acc_pos: &mut Vec<u32>) {
    acc_pos.push(0);
    for i in 0..line.len() {
        acc_pos.push(acc_pos[i] + line[i]);
    }
}

fn fill_queue(
    line: &[u32],
    acc_pos: &Vec<u32>,
    idx_lut: &mut Vec<PriorityQueue<usize, Reverse<usize>>>,
) {
    for i in (1..line.len()).step_by(2) {
        let free_size = line[i] as usize;
        if free_size > 0 {
            idx_lut[line[i] as usize - 1].push(acc_pos[i] as usize, Reverse(acc_pos[i] as usize));
        }
    }
}

fn find_first_freespace(
    idx_lut: &Vec<PriorityQueue<usize, Reverse<usize>>>,
    max_pos: usize,
    mem_size: usize,
) -> (usize, usize) {
    let mut freespace_size = 0;
    let mut first_freespace = max_pos;
    for j in mem_size - 1..9 {
        if let Some(q) = idx_lut.get(j) {
            if !q.is_empty() && q.peek().unwrap().0 < &first_freespace {
                freespace_size = j + 1;
                first_freespace = *q.peek().unwrap().0;
            }
        }
    }
    (freespace_size, first_freespace)
}

fn pop_freespace(idx_lut: &mut Vec<PriorityQueue<usize, Reverse<usize>>>, mem_size: usize) {
    idx_lut.get_mut(mem_size - 1).unwrap().pop();
}

fn push_freespace(
    idx_lut: &mut Vec<PriorityQueue<usize, Reverse<usize>>>,
    freespace_left: usize,
    first_freespace: usize,
) {
    idx_lut
        .get_mut(freespace_left - 1)
        .unwrap()
        .push(first_freespace, Reverse(first_freespace));
}

fn parse_input(filename: &str) -> Vec<u32> {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u32)
        .collect()
}
