use ndarray::Array2;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

use ndarray_stats::QuantileExt;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Connection {
    from: String,
    to: String,
}

pub fn run(filename: &str) -> io::Result<()> {
    test_code();
    let (connections, nodes) = parse_input(filename);
    println!("Star 1: {}", star1(&connections, &nodes));
    println!("Star 2: {}", star2(&connections, &nodes));

    Ok(())
}

fn test_code() {
    let test_inputs = Vec::from([
        "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka", "wh-tc",
        "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka", "td-qp", "aq-cg",
        "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de", "kh-ta", "co-tc", "wh-qp",
        "tb-vc", "td-yn",
    ]);
    let test_nodes = Vec::from([
        "kh", "tc", "qp", "de", "cg", "ka", "co", "yn", "aq", "ub", "tb", "vc", "wh", "ta", "td",
        "wq",
    ]);
    let mut connections = Vec::new();
    let mut added_nodes = HashMap::new();
    for &line in test_inputs.iter() {
        parse_line(line, &mut connections, &mut added_nodes);
    }
    assert_eq!(test_inputs.len(), connections.len());
    assert_eq!(test_nodes.len(), added_nodes.len());

    let mut nodes_names = vec![""; added_nodes.len()];
    for (name, &idx) in added_nodes.iter() {
        nodes_names[idx] = name;
    }
    assert_eq!(nodes_names, test_nodes);

    let adjacency_map = create_adjacency_map(&connections, &added_nodes);
    let nodes_part_of_cycle = find_nth_cycle(&adjacency_map, 3);
    let cycles = produce_3_cycles(&nodes_part_of_cycle, &adjacency_map);
    assert_eq!(n_cycles_with_t(&cycles, &nodes_names), 7);

    let max_clique = find_best_max_clique(&adjacency_map, &nodes_names);
    assert_eq!(max_clique, "co,de,ka,ta");
}

fn star1(connections: &[Connection], nodes: &HashMap<String, usize>) -> i64 {
    let adjacency_map = create_adjacency_map(connections, nodes);
    let nodes_part_of_cycle = find_nth_cycle(&adjacency_map, 3);
    let cycles = produce_3_cycles(&nodes_part_of_cycle, &adjacency_map);
    let mut nodes_names = vec![""; nodes.len()];
    for (name, &idx) in nodes.iter() {
        nodes_names[idx] = name;
    }
    n_cycles_with_t(&cycles, &nodes_names)
}

fn star2(connections: &[Connection], nodes: &HashMap<String, usize>) -> String {
    let adjacency_map = create_adjacency_map(connections, nodes);
    let mut nodes_names = vec![""; nodes.len()];
    for (name, &idx) in nodes.iter() {
        nodes_names[idx] = name;
    }
    find_best_max_clique(&adjacency_map, &nodes_names)
}

fn find_best_max_clique(adjacency_map: &Array2<i32>, node_names: &[&str]) -> String {
    let reduced_adjacency_map = get_reduced_adjacency_map(adjacency_map);
    let mut max_clique = Vec::new();
    for i in 0..adjacency_map.nrows() {
        let local_max_clique = find_maximal_clique(&reduced_adjacency_map, i);
        if local_max_clique.len() > max_clique.len() {
            max_clique = local_max_clique;
        }
    }
    let mut names = Vec::new();
    for i in max_clique {
        names.push(node_names[i]);
    }
    names.sort();
    let password = names.join(",");
    password
}

fn get_reduced_adjacency_map(adjacency_map: &Array2<i32>) -> Vec<Vec<usize>> {
    let mut reduced_adjacency_map = Vec::new();
    for i in 0..adjacency_map.nrows() {
        let mut neighbors = Vec::new();
        for j in 0..adjacency_map.ncols() {
            if adjacency_map[[i, j]] > 0 {
                neighbors.push(j);
            }
        }
        reduced_adjacency_map.push(neighbors);
    }

    reduced_adjacency_map
}

fn find_maximal_clique(reduced_adjacency_map: &Vec<Vec<usize>>, n: usize) -> Vec<usize> {
    let mut clique = Vec::new();
    clique.push(n);

    for (i, neighbors) in reduced_adjacency_map.iter().enumerate() {
        if i != n && clique.iter().all(|&neighbor| neighbors.contains(&neighbor)) {
            clique.push(i);
        }
    }

    clique
}

fn n_cycles_with_t(cycles: &[Vec<usize>], node_names: &[&str]) -> i64 {
    let mut sum = 0;
    for cycle in cycles {
        let mut v = Vec::new();
        let mut contains_t = false;
        for &node in cycle.iter() {
            if node_names[node][0..1] == *"t" {
                contains_t = true;
            }
            v.push(node_names[node]);
        }
        if contains_t {
            sum += 1;
        }
    }

    sum / 2
}

fn produce_3_cycles(nodes_part_of_cycle: &[usize], adjacency_map: &Array2<i32>) -> Vec<Vec<usize>> {
    let mut cycles = Vec::new();
    for (i, &node) in nodes_part_of_cycle.iter().enumerate() {
        for (j, &node2) in nodes_part_of_cycle.iter().enumerate() {
            if i <= j {
                continue;
            }
            for (k, &node3) in nodes_part_of_cycle.iter().enumerate() {
                if k <= j {
                    continue;
                }
                if adjacency_map[[node, node2]] == 0
                    || adjacency_map[[node2, node3]] == 0
                    || adjacency_map[[node3, node]] == 0
                {
                    continue;
                }
                let mut cycle = Vec::new();
                cycle.push(node);
                cycle.push(node2);
                cycle.push(node3);
                cycles.push(cycle);
            }
        }
    }

    cycles
}

fn find_nth_cycle(adjacency_map: &Array2<i32>, n: usize) -> Vec<usize> {
    assert!(n > 0);
    let mut output = adjacency_map.clone();
    for _ in 0..n - 1 {
        for i in 0..output.nrows() {
            output[[i, i]] = 0;
        }
        output = output.clamp(0, 1).dot(adjacency_map);
        //println!("{:?}", output);
    }
    let mut nodes_part_of_cycle = Vec::new();
    for i in 0..output.nrows() {
        if output[[i, i]] > 0 {
            nodes_part_of_cycle.push(i);
        }
    }
    nodes_part_of_cycle
}

fn create_adjacency_map(connections: &[Connection], nodes: &HashMap<String, usize>) -> Array2<i32> {
    let mut adjacency_map = Array2::zeros((nodes.len(), nodes.len()));

    for connection in connections {
        let &from_idx = nodes.get(&connection.from).unwrap();
        let &to_idx = nodes.get(&connection.to).unwrap();
        adjacency_map[[from_idx, to_idx]] = 1;
        adjacency_map[[to_idx, from_idx]] = 1;
    }

    adjacency_map
}

fn parse_input(filename: &str) -> (Vec<Connection>, HashMap<String, usize>) {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut connections = Vec::new();
    let mut added_nodes = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        parse_line(&line, &mut connections, &mut added_nodes);
    }

    (connections, added_nodes)
}

fn parse_line(
    line: &str,
    connections: &mut Vec<Connection>,
    added_nodes: &mut HashMap<String, usize>,
) {
    let node1 = line[0..2].to_string();
    let node2 = line[3..5].to_string();
    let conncetion = Connection {
        from: node1.clone(),
        to: node2.clone(),
    };
    let mut nodes_alredy_added = added_nodes.len();
    if !added_nodes.contains_key(&node1) {
        added_nodes.insert(node1, nodes_alredy_added);
        nodes_alredy_added += 1;
    }
    if !added_nodes.contains_key(&node2) {
        added_nodes.insert(node2, nodes_alredy_added);
    }

    connections.push(conncetion);
}
