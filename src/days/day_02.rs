#![warn(clippy::all, clippy::pedantic)]
use std::fs;

pub fn main() {
    let rps :u64 = fs::read_to_string("./data/day_02.txt")
        .expect("asd")
        .lines()
        .map(translate)
        .map(score)
        .sum();
    println!("{:?}", rps);
}

fn score(s: &str) -> u64 {
    match s {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => 0,
    }
}

fn translate(s: &str) -> &str {
    match s {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",
        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        _ => "",
    }
}