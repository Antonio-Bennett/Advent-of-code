#![allow(dead_code)]

use std::io;

pub fn part_a(input: &str) -> io::Result<i64> {
    let input: Vec<&str> = input.trim().split('\n').collect();
    let mut count = vec![0; input[0].len()];
    let mut gamma_string = String::new();
    let mut epsilon_string = String::new();
    let gamma: i64;
    let epsilon: i64;

    input.iter().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, val)| match val {
            '0' => count[idx] -= 1,
            '1' => count[idx] += 1,
            _ => panic!(),
        });
    });

    count.iter().for_each(|count| {
        if *count < 0 {
            gamma_string += "0";
            epsilon_string += "1";
        } else {
            gamma_string += "1";
            epsilon_string += "0";
        }
    });

    gamma = i64::from_str_radix(&gamma_string, 2).unwrap();
    epsilon = i64::from_str_radix(&epsilon_string, 2).unwrap();
    // epsilon = !gamma; <-- This does not work. Thought I could flip bits with bitwise not

    Ok(gamma * epsilon)
}
