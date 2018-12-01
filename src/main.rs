use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, ErrorKind};


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

    for i in changes.clone() {
        freq += i;
    }
    println!("part1 freq: {:?}", freq);
}
