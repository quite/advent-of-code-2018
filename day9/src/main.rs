extern crate tools;
use tools::read_from_file;

extern crate regex;
use regex::Regex;

use std::collections::VecDeque;

fn rot(circle: &mut VecDeque<u32>, n: i32) {
    for _ in 0..n.abs() {
        if n < 0 {
            let t = circle.pop_back().unwrap();
            circle.push_front(t);
        } else {
            let t = circle.pop_front().unwrap();
            circle.push_back(t);
        }
    }
}

fn play(num: u32, last: u32) -> u32 {
    let mut scores = vec![0; num as usize];

    let mut circle: VecDeque<u32> = VecDeque::with_capacity(last as usize);

    circle.push_back(0);

    for marbl in 1..=last {
        if (marbl % 23) == 0 {
            rot(&mut circle, -7);
            let player = ((marbl - 1) % num) as usize;
            scores[player] += circle.pop_back().unwrap() + marbl;
            rot(&mut circle, 1);
        } else {
            rot(&mut circle, 1);
            circle.push_back(marbl);
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let input = read_from_file("input").unwrap();

    let re = Regex::new(r"^(?P<num>\d+) players; last marble is worth (?P<p>\d+) points$").unwrap();

    let l = input.lines().next().unwrap();
    let cap = re.captures(l).unwrap();
    let num: u32 = cap["num"].parse().unwrap();
    let last: u32 = cap["p"].parse().unwrap();

    println!("part1 winning score: {:?}", play(num, last));
    println!("part2 winning score: {:?}", play(num, last * 100));
}
