extern crate tools;
use tools::read_from_file;

fn react(polymer: &str) -> String {
    let mut unititer = polymer.chars().peekable();
    let mut reacted = String::new();
    loop {
        let curr = unititer.next();
        let next = unititer.peek().cloned();
        match (curr, next) {
            (Some(c), Some(n)) => {
                // fits aA, Aa, but not AA, aa
                if (c.is_lowercase() && n.is_uppercase() && c.to_ascii_uppercase() == n)
                    || (c.is_uppercase() && n.is_lowercase() && c == n.to_ascii_uppercase())
                {
                    unititer.next();
                } else {
                    reacted.push(c);
                }
            }
            (Some(c), None) => {
                reacted.push(c);
            }
            _ => break,
        }
    }
    reacted
}

fn fully_react(polymer: &str) -> String {
    let mut polymer = polymer.trim().to_string();
    loop {
        let reacted = react(&polymer);
        if reacted == polymer {
            break;
        }
        polymer = reacted;
    }
    polymer
}

fn main() {
    let polymer = read_from_file("input").unwrap().trim().to_string();

    let reacted = fully_react(&polymer);
    println!("part1 units remaining: {}", reacted.len());
}
