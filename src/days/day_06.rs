use std::collections::HashSet;
use std::fs;

pub fn main() {
    let f = fs::read_to_string("./data/day_06.txt")
        .expect("asd");

    let msg_size = 14;

    for i in 0..f.len() {
        let hs: HashSet<char> = f[i..i+msg_size]
            .chars()
            .collect();

        if hs.len() == msg_size {
            println!("{}", i+msg_size);
            break;
        }
    }
}