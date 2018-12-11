use std::ops::RangeInclusive;

#[derive(Debug, Eq, PartialEq)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct Square {
    pos: Pos,
    size: u32,
}

fn get_cellpower(pos: &Pos, serial: u32) -> Result<i32, &'static str> {
    if pos.x < 1 || pos.x > 300 || pos.y < 1 || pos.y > 300 {
        return Err("out of bounds");
    }
    let rack_id = pos.x + 10;
    let mut power = rack_id as i32 * pos.y as i32;
    power += serial as i32;
    power *= rack_id as i32;
    power = power / 100 % 10;
    power -= 5;
    Ok(power)
}

fn get_squarepower(sq: &Square, serial: u32) -> i32 {
    let mut power = 0;
    for x in sq.pos.x..=sq.pos.x + (sq.size - 1) {
        for y in sq.pos.y..=sq.pos.y + (sq.size - 1) {
            power += get_cellpower(&Pos { x, y }, serial).unwrap();
        }
    }
    power
}

fn get_maxpowersq(serial: u32, sizes: RangeInclusive<u32>) -> (Square, i32) {
    let mut sq: Option<Square> = None;
    let mut maxpower: i32 = 0;

    for sq_size in sizes {
        for x in 1..=300 - (sq_size - 1) {
            for y in 1..=300 - (sq_size - 1) {
                let power = get_squarepower(
                    &Square {
                        pos: Pos { x, y },
                        size: sq_size,
                    },
                    serial,
                );
                if power > maxpower {
                    sq = Some(Square {
                        pos: Pos { x, y },
                        size: sq_size,
                    });
                    maxpower = power;
                }
            }
        }
    }
    (sq.unwrap(), maxpower)
}

fn main() {
    assert_eq!(get_cellpower(&Pos { x: 3, y: 5 }, 8).unwrap(), 4);
    assert_eq!(get_cellpower(&Pos { x: 122, y: 79 }, 57).unwrap(), -5);
    assert_eq!(get_cellpower(&Pos { x: 217, y: 196 }, 39).unwrap(), 0);
    assert_eq!(get_cellpower(&Pos { x: 101, y: 153 }, 71).unwrap(), 4);
    assert_eq!(get_maxpowersq(18, 3..=3), (Square { pos: Pos { x: 33, y: 45 }, size: 3 }, 29));
    assert_eq!(get_maxpowersq(42, 3..=3), (Square { pos: Pos { x: 21, y: 61 }, size: 3 }, 30));

    // indeed lazily cut off range
    assert_eq!(get_maxpowersq(18, 1..=16), (Square { pos: Pos { x: 90, y: 269 }, size: 16 }, 113));
    assert_eq!(get_maxpowersq(42, 1..=12), (Square { pos: Pos { x: 232, y: 251 }, size: 12 }, 119));

    let grid_serial = 2187;

    let (sq, maxpower) = get_maxpowersq(grid_serial, 3..=3);
    println!(
        "part1 coord: {},{} (with power {})",
        sq.pos.x, sq.pos.y, maxpower
    );

    let (sq, maxpower) = get_maxpowersq(grid_serial, 1..=300);
    println!(
        "part2 coord,size: {},{},{} (with power {})",
        sq.pos.x, sq.pos.y, sq.size, maxpower
    );
}
