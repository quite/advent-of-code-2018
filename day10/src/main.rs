extern crate tools;
use tools::read_from_file;

extern crate ncurses;
extern crate regex;

use ncurses::*;
use regex::Regex;

#[derive(Debug)]
struct Point {
    xpos: i32,
    ypos: i32,
    xvel: i32,
    yvel: i32,
}

impl Point {
    fn moveit(&mut self) {
        self.xpos += self.xvel;
        self.ypos += self.yvel;
    }
}

fn line_to_point(line: &str) -> Point {
    let re = Regex::new(
        r"^(?x)
           position=<\s*(?P<xpos>-?\d+),\s*(?P<ypos>-?\d+)>\s*
           velocity=<\s*(?P<xvel>-?\d+),\s*(?P<yvel>-?\d+)>$",
    ).unwrap();

    let cap = re.captures(line).unwrap();
    let xpos: i32 = cap["xpos"].parse().unwrap();
    let ypos: i32 = cap["ypos"].parse().unwrap();
    let xvel: i32 = cap["xvel"].parse().unwrap();
    let yvel: i32 = cap["yvel"].parse().unwrap();
    Point {
        xpos,
        ypos,
        xvel,
        yvel,
    }
}

fn main() {
    let input = read_from_file("input").unwrap();

    let mut points = Vec::new();
    for l in input.lines() {
        points.push(line_to_point(l));
    }

    initscr();
    keypad(stdscr(), true);
    noecho();

    let mut t = 0;
    loop {
        for p in points.iter_mut() {
            p.moveit();
        }
        t += 1;

        let ymax = points.iter().max_by_key(|p| p.ypos).unwrap().ypos;
        let xmax = points.iter().max_by_key(|p| p.xpos).unwrap().xpos;

        // adjusted after ocular inspection in this little plot lab
        if ymax < 400 && xmax < 400 {
            for p in points.iter() {
                mv(p.ypos, p.xpos);
                addch('*' as chtype);
            }
            mvprintw(0, 0, "q:quit any:next");
            if getch() == 'q' as i32 {
                break;
            }
            clear();
        }
    }

    endwin();
    println!("seconds: {}", t);
}
