#![allow(dead_code)]

use std::io;

pub fn part_a(input: &str) -> io::Result<usize> {
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|token| token.parse().ok())
        .collect();

    Ok(numbers.windows(2).filter(|pair| pair[0] < pair[1]).count())
}
