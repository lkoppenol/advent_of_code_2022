#![warn(clippy::all, clippy::pedantic)]

extern crate core;

use std::fs;
use regex::Regex;
use regex::Captures;

pub fn main() {
    let f = fs::read_to_string("./data/day_05.txt")
        .expect("asd");
    let lines = f.lines();

    let mut stacks= [
        str_to_vec("GBDCPR"),
        str_to_vec("GVH"),
        str_to_vec("MPJDQSN"),
        str_to_vec("MNCDGLSP"),
        str_to_vec("SLFPCNBJ"),
        str_to_vec("STGVZDBQ"),
        str_to_vec("QTFHMZB"),
        str_to_vec("FBDMC"),
        str_to_vec("GQCF")
    ];

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines {
        let caps = re.captures(line).unwrap();
        let amount = group_to_int(&caps, 1);
        let from = group_to_int(&caps, 2);
        let to = group_to_int(&caps, 3);

        let mut es: Vec<char> = stacks[from-1].drain(0..amount).collect();
        // es.reverse();
        es.append(&mut stacks[to-1]);
        stacks[to-1] = es;
    }
    println!();

    for s in &stacks {
        println!("{:?}", s);
    }
}


fn group_to_int(caps: &Captures, i: usize) -> usize {
    let i = caps
        .get(i)
        .expect("|")
        .as_str()
        .parse::<usize>()
        .expect(",,|,,");
    i
}

fn str_to_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}