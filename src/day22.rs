use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(filename: &str) -> io::Result<()> {
    let secrets = parse_input(filename);
    test_code();
    println!("Star 1: {}", star1(&secrets));
    println!("Star 2: {}", star2(&secrets));

    Ok(())
}

fn star1(secrets: &[i64]) -> i64 {
    let mut sum = 0;
    for secret in secrets {
        let new_secret = calculate_nth_secret(*secret, 2000);
        sum += new_secret;
    }
    sum
}

fn star2(secrets: &[i64]) -> i64 {
    let mut all_occurences = Vec::new();
    let mut all_patterns = HashSet::new();
    for &secret in secrets {
        let diffs_n_prices = calculate_price_differences(secret, 2000);
        let occurences: HashMap<(i64, i64, i64, i64), i64> = map_occurences(diffs_n_prices);
        all_patterns.extend(occurences.keys().cloned());
        all_occurences.push(occurences);
    }
    let mut sum = 0;
    for pattern in all_patterns {
        let mut tmp_sum = 0;
        for map in &all_occurences {
            tmp_sum += *map.get(&pattern).unwrap_or(&0);
        }
        if tmp_sum > sum {
            sum = tmp_sum;
        }
    }
    sum
}

fn test_code() {
    assert!(calculate_nth_secret(123, 1) == 15887950);
    assert!(calculate_nth_secret(123, 2) == 16495136);
    assert!(calculate_nth_secret(123, 3) == 527345);
    assert!(calculate_nth_secret(123, 4) == 704524);
    assert!(calculate_nth_secret(123, 5) == 1553684);
    assert!(calculate_nth_secret(123, 6) == 12683156);
    assert!(calculate_nth_secret(123, 7) == 11100544);
    assert!(calculate_nth_secret(123, 8) == 12249484);
    assert!(calculate_nth_secret(123, 9) == 7753432);
    assert!(calculate_nth_secret(123, 10) == 5908254);

    assert!(calculate_nth_secret(1, 2000) == 8685429);
    assert!(calculate_nth_secret(10, 2000) == 4700978);
    assert!(calculate_nth_secret(100, 2000) == 15273692);
    assert!(calculate_nth_secret(2024, 2000) == 8667524);

    let differences = calculate_price_differences(123, 9);
    let expected_differences = Vec::from([
        (-3, 0),
        (6, 6),
        (-1, 5),
        (-1, 4),
        (0, 4),
        (2, 6),
        (-2, 4),
        (0, 4),
        (-2, 2),
    ]);

    for ((diff, price), (expected_diff, expected_price)) in
        std::iter::zip(differences.clone(), expected_differences)
    {
        assert!(diff == expected_diff);
        assert!(price == expected_price);
    }

    for (i, ep) in std::iter::zip([1, 2, 3, 2024], [7, 7, 0, 9]) {
        let diffs_n_prices = calculate_price_differences(i, 2000);
        let occurences: HashMap<(i64, i64, i64, i64), i64> = map_occurences(diffs_n_prices);
        assert!(*occurences.get(&(-2, 1, -1, 3)).unwrap_or(&0) == ep);
    }
}

fn map_occurences(diffs_n_prices: Vec<(i64, i64)>) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut mapping = HashMap::new();
    for i in 3..diffs_n_prices.len() {
        let sequence = (
            diffs_n_prices[i - 3].0,
            diffs_n_prices[i - 2].0,
            diffs_n_prices[i - 1].0,
            diffs_n_prices[i].0,
        );
        let price = diffs_n_prices[i].1;
        mapping.entry(sequence).or_insert(price);
    }
    mapping
}

fn calculate_price_differences(secret: i64, n: usize) -> Vec<(i64, i64)> {
    let mut differences = Vec::new();
    let mut old_price = secret % 10;
    let mut new_secret = secret;
    for _ in 0..n {
        new_secret = calculate_new_secret(new_secret);
        let price = new_secret % 10;
        let difference = price - old_price;
        differences.push((difference, price));
        old_price = price;
    }
    differences
}

fn calculate_nth_secret(secret: i64, n: usize) -> i64 {
    let mut new_secret = secret;
    for _ in 0..n {
        new_secret = calculate_new_secret(new_secret)
    }
    new_secret
}

fn calculate_new_secret(secret: i64) -> i64 {
    let mut new_secret = secret;

    let tmp1 = new_secret * 64;
    new_secret = mix(secret, tmp1);
    new_secret = prune(new_secret);

    let tmp2 = new_secret / 32;
    new_secret = mix(new_secret, tmp2);
    new_secret = prune(new_secret);

    let tmp3 = new_secret * 2048;
    new_secret = mix(new_secret, tmp3);
    new_secret = prune(new_secret);

    new_secret
}

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

fn parse_input(filename: &str) -> Vec<i64> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}
