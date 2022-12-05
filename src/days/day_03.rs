#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;
use std::fs;
use std::str::Chars;

pub fn main() {
    let rps = fs::read_to_string("./data/day_03.txt")
        .expect("asd");
    let lines = rps.lines().collect::<Vec<&str>>();

    // let asd: u64 = lines
    //     .map(translate)
    //     .sum();
    //
    // println!("{:?}", asd);

    let len = lines.len();

    let asd :u64 = (0..len).map(|n|
        if n % 3 == 0 {
            let c = get_first_char_intersection_3(lines[n].chars(), lines[n+1].chars(), lines[n+2].chars());
            get_char_priority(c)
        } else {
            0
        }
    ).sum();

    println!("{:?}", asd);
}

fn translate(s: &str) -> u64 {
    let half = s.len() / 2;

    let c = get_first_char_intersection(
        s[..half].chars(),
        s[half..].chars()
    );

    get_char_priority(c)
}

fn get_char_priority(c: char) -> u64 {
    let i = c as u64 - 'A' as u64 + 1;
    if i < 27 {
        i + 26
    } else {
        i - 26 - 6
    }
}

fn get_first_char_intersection(c1: Chars, c2: Chars) -> char {
    let compartment_one = c1.collect::<HashSet<char>>();
    let compartment_two = c2.collect::<HashSet<char>>();

    let overlap = compartment_one.intersection(&compartment_two);

    *overlap.into_iter().next().expect(",|,")
}

fn get_first_char_intersection_3(c1: Chars, c2: Chars, c3: Chars) -> char {
    let a = c1.collect::<HashSet<char>>();
    let b = c2.collect::<HashSet<char>>();
    let c = c3.collect::<HashSet<char>>();

    let overlap = a.intersection(&b).find(|it| c.contains(it));

    *overlap.into_iter().next().expect(",|,")
}
