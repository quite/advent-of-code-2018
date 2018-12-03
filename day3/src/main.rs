use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;
#[macro_use]
extern crate itertools;

fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let f = File::open(path)?;
    let f = BufReader::new(f);
    let mut v = Vec::new();
    for line in f.lines() {
        let line = line?;
        v.push(line);
    }
    Ok(v)
}

fn line_to_claim(line: &str) -> Option<Claim> {
    let parsed: Vec<i32> = line
        .split(&['#', '@', ',', ':', 'x'][..])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    match parsed.as_slice() {
        [id, x, y, w, h] => Some(Claim {
            id: *id,
            x: *x,
            y: *y,
            w: *w,
            h: *h,
        }),
        _ => None,
    }
}

struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() {
    let lines = read_lines("input").expect("read lines");

    let mut claims: Vec<Claim> = Vec::new();

    for l in lines.iter() {
        claims.push(line_to_claim(l).unwrap());
    }

    let mut overlaps = HashMap::new();
    for c in claims.iter() {
        for patch in iproduct!(c.x..c.x + c.w, c.y..c.y + c.h) {
            let entry = overlaps.entry(patch).or_insert(0);
            *entry += 1;
        }
    }

    println!(
        "part1 overlapping squares: {:?}",
        overlaps
            .iter()
            .filter(|&(_patch, count)| *count >= 2)
            .count()
    );
}
