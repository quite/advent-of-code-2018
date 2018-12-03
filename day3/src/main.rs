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

fn line_to_rect(line: &str) -> Option<Rectangle> {
    let parsed: Vec<i32> = line
        .split(&['#', '@', ',', ':', 'x'][..])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    match parsed.as_slice() {
        [_id, x, y, w, h] => Some(Rectangle {
            x: *x,
            y: *y,
            w: *w,
            h: *h,
        }),
        _ => None,
    }
}

struct Rectangle {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() {
    let lines = read_lines("input").expect("read lines");

    let mut rects: Vec<Rectangle> = Vec::new();

    for l in lines.iter() {
        rects.push(line_to_rect(l).unwrap());
    }

    let mut overlaps = HashMap::new();
    for r in rects.iter() {
        for patch in iproduct!(r.x..r.x + r.w, r.y..r.y + r.h) {
            let entry = overlaps.entry(patch).or_insert(0);
            *entry += 1;
        }
    }

    println!(
        "part1 overlapping claims: {:?}",
        overlaps
            .iter()
            .filter(|&(_patch, count)| *count >= 2)
            .count()
    );
}
