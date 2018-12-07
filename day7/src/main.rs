extern crate tools;
use tools::read_from_file;

use std::collections::HashMap;
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

extern crate linked_hash_set;
use linked_hash_set::LinkedHashSet;

fn main() {
    let input = read_from_file("input").unwrap();

    // map step -> requirements
    let mut stepreqs: HashMap<char, HashSet<char>> = HashMap::new();

    let re = Regex::new(r"^Step (?P<req>.) m.*p (?P<id>.) can begin\.$").unwrap();
    for l in input.lines() {
        let cap = re.captures(l).unwrap();
        let id: &char = &cap["id"].chars().next().unwrap();
        let req: &char = &cap["req"].chars().next().unwrap();
        // add this step and the new requirement
        stepreqs
            .entry(id.clone())
            .or_insert(HashSet::new())
            .insert(*req);
        // if the req is not already a step, store it without reqs
        stepreqs.entry(req.clone()).or_insert(HashSet::new());
    }

    // set of steps done, ordered
    let mut done: LinkedHashSet<char> = LinkedHashSet::new();

    while !stepreqs.is_empty() {
        for c in b'A'..=b'Z' {
            let step = c as char;
            if let Some(reqs) = stepreqs.get(&step).cloned() {
                if reqs.is_empty() {
                    stepreqs.remove(&step);
                    done.insert(step);
                    // remove the step from all steps' requirement sets
                    for (_, r) in stepreqs.iter_mut() {
                        r.retain(|&s| s != step);
                    }
                    // restart in alphabetic order
                    break;
                }
            }
        }
    }

    let s: String = done.into_iter().collect();
    println!("step1 order: {:?}", s);
}
