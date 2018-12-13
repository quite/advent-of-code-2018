extern crate tools;
use tools::read_from_file;

use std::collections::HashMap;

fn run(initial: &str, rules: &HashMap<&str, char>, gens: usize) -> isize {
    let mut state: String = format!("...{}...", initial.to_string());
    let mut score: isize = 0;
    let mut last_score: isize = 0;
    let mut delta: isize = 0;
    let mut last_delta: isize = 0;
    let mut delta_count: usize = 0;

    for g in 1..=gens {
        // println!("{:2}: {}", g, state);
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

        score = state
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            // state begins with 3 dots, and 1 dot per generation
            .map(|(i, _)| (i - 3 - gens) as isize)
            .sum();

        delta = score - last_score;
        if delta == last_delta {
            delta_count += 1;
        } else {
            delta_count = 0;
        }

        // "good value" for detecting stable monotonous increase
        if delta_count > 100 {
            score += delta * (gens - g) as isize;
            last_score = score;
            break;
        }

        last_score = score;
        last_delta = delta;
    }

    last_score
}

fn main() {
    let input = read_from_file("input").unwrap();
    let mut lines = input.lines();

    let initial: &str = lines
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

    println!("part1 score: {:?}", run(&initial, &rules, 20));
    println!("part2 score: {:?}", run(&initial, &rules, 5_000_000_000));
}
