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

// Returns true if any letter occurs exactly n times in string s
fn exactly_n(s: &String, n: i32) -> bool {
    let mut counts = HashMap::new();
    for letter in s.chars() {
        let entry = counts.entry(letter).or_insert(0);
        *entry += 1;
    }
    for (_letter, count) in counts.iter() {
        if *count == n {
            return true;
        }
    }
    false
}

// Returns char indices at which strings a and b differ
fn diff_indices(a: &String, b: &String) -> Vec<usize> {
    a.chars()
        .zip(b.chars())
        .enumerate()
        .filter(|&(_i, (a, b))| a != b)
        .map(|(i, (_, _))| i)
        .collect()
}

fn main() {
    let lines = read_lines("input").expect("read lines");

    let (mut pairs, mut triples) = (0, 0);
    for line in lines.iter() {
        if exactly_n(&line, 2) {
            pairs += 1;
        }
        if exactly_n(&line, 3) {
            triples += 1;
        }
    }
    println!("part1 checksum: {:?}", pairs * triples);

    'out: for a in lines.iter() {
        for b in lines.iter() {
            let is = diff_indices(&a, &b);
            if is.len() == 1 {
                let mut s = b.clone();
                s.remove(is[0]);
                println!("part2 letters: {:?}", s);
                break 'out;
            }
        }
    }
}

mod tests {
    #[test]
    fn internal() {
        use super::*;
        assert!(exactly_n(&"abcdef".to_string(), 2) == false);
        assert!(exactly_n(&"abcdef".to_string(), 3) == false);

        assert!(exactly_n(&"bababc".to_string(), 2));
        assert!(exactly_n(&"bababc".to_string(), 3));

        assert!(exactly_n(&"aabcdd".to_string(), 2));
        assert!(exactly_n(&"aabcdd".to_string(), 3) == false);

        assert!(exactly_n(&"abcdee".to_string(), 2));
        assert!(exactly_n(&"abcdee".to_string(), 3) == false);

        assert!(exactly_n(&"ababab".to_string(), 2) == false);
        assert!(exactly_n(&"ababab".to_string(), 3));
    }
}
