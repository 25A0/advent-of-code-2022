use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;
use regex::Regex;

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
struct Coord {
    x: isize,
    y: isize,
}

fn update_tail(head: &Coord, tail: &mut Coord) {
    let delta_x = head.x - tail.x;
    let delta_y = head.y - tail.y;

    if delta_x.abs() > 1 || delta_y.abs() > 1 {
        let mx = delta_x.signum();
        let my = delta_y.signum();
        tail.x += mx;
        tail.y += my;
    }
}

fn main() {

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    let mut head = Coord {x: 0, y: 0};
    let mut tail = Coord {x: 0, y: 0};
    let mut visited = HashSet::new();
    visited.insert(tail);

    // Example line:
    // R 5
    let re = Regex::new(r"([RULD]) (\d+)").expect("Could not compile regex");

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {

            // Parse line with regex
            if let Some(cap) = re.captures(&line) {
                let direction = &cap[1];
                let count = cap[2].parse::<usize>().expect("Could not parse crate count");
                for _ in 0..count {
                    let dx = match direction {
                        "L" => -1,
                        "R" =>  1,
                        _ => 0,
                    };
                    let dy = match direction {
                        "D" => -1,
                        "U" =>  1,
                        _ => 0,
                    };
                    head.x += dx;
                    head.y += dy;
                    update_tail(&head, &mut tail);
                    visited.insert(tail);
                }
            }
        }
    }

    println!("Visited {} locations", visited.len());
}
