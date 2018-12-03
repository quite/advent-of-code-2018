extern crate tools;
use tools::read_from_file;

use std::collections::HashMap;
#[macro_use]
extern crate itertools;

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
    let input = read_from_file("input").unwrap();

    let mut claims: Vec<Claim> = Vec::new();

    for l in input.lines() {
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

    for c in claims.iter() {
        if iproduct!(c.x..c.x + c.w, c.y..c.y + c.h)
            .map(|p| overlaps.get(&p).unwrap())
            .filter(|&i| *i >= 2)
            .sum::<i32>()
            == 0
        {
            println!("part2 non-overlapping claim: {:?}", c.id);
        }
    }
}
