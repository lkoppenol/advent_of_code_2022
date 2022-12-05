#![warn(clippy::all, clippy::pedantic)]

use std::fs;

pub fn main() {
    let f = fs::read_to_string("./data/day_04.txt")
        .expect("asd");

    let rps = f
        .lines()
        .map(handle)
        .filter(|e| *e)
        .count();

    println!("{:?}", rps);

}

fn handle(s: &str) -> bool {
    let l = s
        .split(',')
        .flat_map(|x| x.split('-'))
        .map(|i| i.parse::<u64>().expect(",|,"))
        .collect::<Vec<u64>>();
    check_overlap(&l) | check_overlap2(&l)
}

fn check_overlap(l: &[u64]) -> bool {
    ((l[0] <= l[2]) & (l[1] >= l[3])) | ((l[2] <= l[0]) & (l[3] >= l[1]))
}

fn check_overlap2(l: &[u64]) -> bool {
    ((l[2] <= l[0]) & (l[0] <= l[3])) | ((l[2] <= l[1]) & (l[1] <= l[3]))
}
