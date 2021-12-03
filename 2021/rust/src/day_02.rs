#![allow(dead_code)]

use std::io;

pub fn part_a(input: &str) -> io::Result<i64> {
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    input.trim().split('\n').for_each(|line| {
        let mut line_itr = line.split(' ');
        let cmd = line_itr.next().unwrap();
        let num: i64 = line_itr.next().unwrap().parse().unwrap();

        match cmd {
            "forward" => x += num,
            "down" => y += num,
            "up" => y -= num,
            _ => panic!("Unexpected error"),
        }
    });

    Ok(x * y)
}

pub fn part_b(input: &str) -> io::Result<i64> {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut aim: i64 = 0;

    input.trim().split('\n').for_each(|line| {
        let mut line_itr = line.split(' ');
        let cmd = line_itr.next().unwrap();
        let num: i64 = line_itr.next().unwrap().parse().unwrap();

        match cmd {
            "forward" => {
                x += num;
                y += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => panic!("Unexpected error"),
        }
    });

    Ok(x * y)
}
