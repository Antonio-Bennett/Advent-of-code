#![allow(dead_code)]

use std::io;

pub fn part_a(input: &str) -> io::Result<usize> {
    let numbers: Vec<i32> = input
        .split('\n')
        .filter_map(|token| token.parse().ok())
        .collect();

    Ok(numbers.windows(2).filter(|pair| pair[0] < pair[1]).count())
}

pub fn part_b(input: &str) -> io::Result<usize> {
    let numbers: Vec<i32> = input
        .split('\n')
        .filter_map(|token| token.parse().ok())
        .collect();

    let nums: Vec<i32> = numbers
        .windows(3)
        .map(|chunk| chunk[0] + chunk[1] + chunk[2])
        .collect();

    Ok(nums.windows(2).filter(|pair| pair[0] < pair[1]).count())
}
