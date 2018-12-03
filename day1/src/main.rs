extern crate tools;
use tools::read_from_file;

use std::collections::HashSet;

fn main() {
    let changes: Vec<i32> = read_from_file("input")
        .unwrap()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let mut seen = HashSet::new();
    let mut freq: i32 = 0;

    for i in changes.iter() {
        freq += i;
        seen.insert(freq);
    }
    println!("part1 freq: {:?}", freq);

    'out: loop {
        for i in changes.iter() {
            freq += i;
            if seen.contains(&freq) {
                println!("part2 freq: {:?}", freq);
                break 'out;
            } else {
                seen.insert(freq);
            }
        }
    }
}
