use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

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

struct Rectangle {
    id: i32,
    x: i32,
    y: i32,
    x2: i32,
    y2: i32,
}

fn main() {
    let lines = read_lines("input").expect("read lines");

    let mut rects: Vec<Rectangle> = Vec::new();

    for l in lines.iter() {
        let parsed: Vec<i32> = l
            .split(&['#', '@', ',', ':', 'x'][..])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        match parsed.as_slice() {
            [id, x, y, w, h] => rects.push(Rectangle {
                id: *id,
                x: *x,
                y: *y,
                x2: *x + *w,
                y2: *y + *h,
            }),
            _ => (),
        }
    }

    for r in rects.iter() {
        println!("{:?} {:?} {:?} {:?} {:?}", r.id, r.x, r.y, r.x2, r.y2);
    }
}
