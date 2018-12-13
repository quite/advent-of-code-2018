extern crate tools;
use tools::read_from_file;

use std::collections::HashMap;

fn main() {
    let input = read_from_file("input").unwrap();
    let mut lines = input.lines();

    let init: &str = lines
        .next()
        .unwrap()
        .split(':')
        .map(str::trim)
        .collect::<Vec<&str>>()[1];

    let mut rules: HashMap<&str, char> = HashMap::new();
    for l in lines {
        let mut spl: Vec<&str> = l.split("=>").map(str::trim).collect();
        if spl.len() == 2 {
            rules.insert(spl[0], if spl[1] == "#" { '#' } else { '.' });
        }
    }

    if rules.len() != 32 {
        panic!("We should have exactly 32 rules");
    }

    let gens = 20;

    let mut state: String = format!("...{}...", init.to_string());

    for _ in 1..=gens {
        let mut new = String::from("...");
        {
            let windit = state.as_bytes().windows(5);
            for win in windit {
                let from = String::from_utf8(win.to_vec()).unwrap();
                let to = rules.get(&from.as_ref()).unwrap();
                new.push(*to);
            }
        }
        new.push_str("...");
        state = new;
    }

    let score: isize = state
        .chars()
        .enumerate()
        .filter(|(_, c)| c == &'#')
        // state begins with 3 dots, and 1 dot per generation
        .map(|(i, _)| (i - 3 - gens) as isize)
        .sum();

    println!("part1 score: {:?}", score);
}
