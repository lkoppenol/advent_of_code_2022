#![warn(clippy::all, clippy::pedantic)]
use std::fs;

pub fn main() {
    let mut elves = fs::read_to_string("./data/day_01.txt")
        .expect("asd")
        .split("\n\n")
        .into_iter()
        .map(parse_elf)
        .collect::<Vec<i32>>();

    elves.sort_unstable();

    let max = elves.last().unwrap_or(&0);
    let top_3 = &elves[elves.len()-3..].iter().sum::<i32>();

    println!("{:?}", max);
    println!("{:?}", top_3);
}

fn parse_elf(elf: &str) -> i32 {
    elf
        .split('\n')
        .map(parse_number)
        .into_iter()
        .sum::<i32>()
}

fn parse_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or(0)
}
