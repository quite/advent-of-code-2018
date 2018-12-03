extern crate tools;
use tools::read_from_file;

use std::collections::HashMap;

// Returns true if any letter occurs exactly n times in string s
fn exactly_n(s: &str, n: i32) -> bool {
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
fn diff_indices(a: &str, b: &str) -> Vec<usize> {
    a.chars()
        .zip(b.chars())
        .enumerate()
        .filter(|&(_i, (a, b))| a != b)
        .map(|(i, (_, _))| i)
        .collect()
}

fn main() {
    let input = read_from_file("input").unwrap();

    let (mut pairs, mut triples) = (0, 0);
    for line in input.lines() {
        if exactly_n(&line, 2) {
            pairs += 1;
        }
        if exactly_n(&line, 3) {
            triples += 1;
        }
    }
    println!("part1 checksum: {:?}", pairs * triples);

    'out: for a in input.lines() {
        for b in input.lines() {
            let is = diff_indices(&a, &b);
            if is.len() == 1 {
                let mut s = b.clone().to_string();
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
