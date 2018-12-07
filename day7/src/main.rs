extern crate tools;
use tools::read_from_file;

use std::collections::HashMap;

extern crate regex;
use regex::Regex;

extern crate linked_hash_set;
use linked_hash_set::LinkedHashSet;

fn main() {
    let input = read_from_file("input").unwrap();

    // map step -> requirements
    let mut stepreqs: HashMap<char, Vec<char>> = HashMap::new();

    let re = Regex::new(r"^Step (?P<req>.) m.*p (?P<id>.) can begin\.$").unwrap();
    for l in input.lines() {
        let cap = re.captures(l).unwrap();
        let id: &char = &cap["id"].chars().next().unwrap();
        let req: &char = &cap["req"].chars().next().unwrap();
        // store/extend this step's requirements
        stepreqs
            .entry(id.clone())
            .or_insert(vec![])
            .extend(vec![req]);
        // if this req is not already a step, store it without reqs
        stepreqs.entry(req.clone()).or_insert(vec![]);
    }

    // set of steps done, ordered
    let mut done: LinkedHashSet<char> = LinkedHashSet::new();

    while !stepreqs.is_empty() {
        let mut todo: Vec<char> = stepreqs.keys().cloned().collect();
        todo.sort();
        for step in todo {
            if let Some(reqs) = stepreqs.get(&step).cloned() {
                // are all reqs for this step done?
                if reqs.iter().filter(|&r| !done.contains(r)).count() == 0 {
                    stepreqs.remove(&step);
                    done.insert(step);
                    // restart in alphabetic order
                    break;
                }
            }
        }
    }

    let s: String = done.into_iter().collect();
    println!("step1 order: {:?}", s);
}
