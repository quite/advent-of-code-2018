extern crate tools;
use tools::read_from_file;

use std::collections::HashMap;

type Point = (u32, u32);

fn mdist(pos: Point, goal: Point) -> u32 {
    ((pos.0 - goal.0) as i32).abs() as u32 + ((pos.1 - goal.1) as i32).abs() as u32
}

fn main() {
    let input = read_from_file("input").unwrap();

    let points: Vec<Point> = input
        .lines()
        .map(|l| l.split(',').map(|s| s.trim().parse().unwrap()).collect())
        .map(|v: Vec<u32>| (v[0], v[1]))
        .collect();

    let xmax = points.iter().max_by_key(|p| p.0).unwrap().0 as usize;
    let ymax = points.iter().max_by_key(|p| p.1).unwrap().1 as usize;

    // grid storing the nearest point (x, y) for each location
    // TODO dynamically allocated using xmax and ymax, not hardcoded big enough
    let mut grid = [[None; 400]; 400];

    let mut areasizes: HashMap<Point, u32> = HashMap::new();

    for x in 0..=xmax {
        for y in 0..=ymax {
            // for every location on grid, get hold of nearest point
            let mut nearpoint: Option<Point> = None;

            let mut neardist = (xmax + ymax) as u32;
            for p in points.iter() {
                let dist = mdist((x as u32, y as u32), *p);
                if dist < neardist {
                    neardist = dist;
                    nearpoint = Some((p.0, p.1));
                } else if dist == neardist {
                    nearpoint = None;
                }
            }

            if let Some(point) = nearpoint {
                grid[x][y] = Some(point);
                *areasizes.entry(point).or_insert(0) += 1
            }
        }
    }

    // follow the 4 borders of infinity and remove points which areas contain
    // locations on the border infinity
    for x in 0..=xmax {
        if let Some(northborder_point) = grid[x][0] {
            areasizes.remove(&northborder_point);
        }
        if let Some(southborder_point) = grid[x][ymax] {
            areasizes.remove(&southborder_point);
        }
    }

    for y in 0..=ymax {
        if let Some(westborder_point) = grid[0][y] {
            areasizes.remove(&westborder_point);
        }
        if let Some(eastborder_point) = grid[xmax][y] {
            areasizes.remove(&eastborder_point);
        }
    }

    let (_, largest) = areasizes.iter().max_by_key(|&(_, locs)| locs).unwrap();
    println!("part1 largest area: {:?}", largest);
}
