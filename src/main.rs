use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use std::collections::HashSet;
use std::process;

fn read_ints(path: &str) -> Result<Vec<i32>, io::Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    let mut v = Vec::new();
    for line in f.lines() {
        let line = line?;
        let i: i32 = line
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(i);
    }

    Ok(v)
}

fn main() {
    let mut freq: i32 = 0;

    let changes = read_ints("input").expect("ints");
    let mut seen = HashSet::new();

    for i in changes.iter() {
        freq += i;
        seen.insert(freq);
    }
    println!("part1 freq: {:?}", freq);

    loop {
        for i in changes.iter() {
            freq += i;
            if seen.contains(&freq) {
                println!("part2 freq: {:?}", freq);
                process::exit(0);
            } else {
                seen.insert(freq);
            }
        }
    }
}
