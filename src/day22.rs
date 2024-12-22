use std::convert::TryInto;
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
        println!("{} -> {}", secret, new_secret);
        sum += new_secret;
    }
    sum
}

fn star2(secrets: &[i64]) -> i64 {
    0
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
}

fn calculate_nth_secret(secret: i64, n: usize) -> i64 {
    let mut new_secret = secret;
    for i in 0..n {
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
