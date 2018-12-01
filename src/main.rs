use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("input not found");
    let f = BufReader::new(f);

    let mut freq: i32 = 0;

    for line in f.lines() {
        let line = line.expect("line read");

        let i: i32 = line.trim().parse().expect("parse number");
        freq += i;
    }

    println!("freq: {:?}", freq);
}
