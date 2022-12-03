use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_numbers(path: &str) -> Vec<i32> {
    let file = File::open(path)
        .expect("File not found");

    let reader = BufReader::new(file);

    let input = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    input
}
