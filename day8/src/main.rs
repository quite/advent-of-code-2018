extern crate tools;
use tools::read_from_file;

fn process(inputints: &mut Vec<u32>, metadatas: &mut Vec<u32>) {
    let mut child_cnt = inputints.remove(0);
    let mut metadata_cnt = inputints.remove(0);
    while child_cnt > 0 {
        process(inputints, metadatas);
        child_cnt -= 1;
    }
    while metadata_cnt > 0 {
        metadatas.push(inputints.remove(0));
        metadata_cnt -= 1;
    }
}

fn process2(inputints: &mut Vec<u32>) -> u32 {
    let mut child_cnt = inputints.remove(0);
    let mut metadatas_cnt = inputints.remove(0);

    let mut childvals: Vec<u32> = Vec::new();
    while child_cnt > 0 {
        childvals.push(process2(inputints));
        child_cnt -= 1;
    }

    let mut metadatas: Vec<u32> = Vec::new();
    while metadatas_cnt > 0 {
        metadatas.push(inputints.remove(0));
        metadatas_cnt -= 1;
    }

    let mut nodeval = 0;
    if childvals.is_empty() {
        nodeval = metadatas.iter().sum::<u32>();
    } else {
        for metadata in metadatas.iter() {
            if let Some(val) = childvals.get((metadata-1) as usize) {
                nodeval += val;
            }
        }
    }
    nodeval
}

fn main() {
    let input = read_from_file("input").unwrap();
    let inputints: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().expect("parse() int failed"))
        .collect();

    let mut metadatas: Vec<u32> = Vec::new();
    process(&mut inputints.clone(), &mut metadatas);
    println!("part1 sum: {:?}", metadatas.iter().sum::<u32>());

    println!("part2 rootnode value: {:?}", process2(&mut inputints.clone()));
}
