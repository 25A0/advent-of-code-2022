// There are two input files. One for the initial crate config,
// ('crate-config.txt') and one for the movement operations ('input.txt').
//
// Approach:
// Recreate the original crate config as an array of vectors.
// There are 9 crate stacks in total.  We can read each line,
// use a regex to find the locations of the crates, and append
// them to the appropriate vector.
//
// Then read the movement operation file line by line. Parse the four numbers
// with regex and apply the operation.
// Finally print the last (topmost) crate of each vector.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn crate_config() -> [Vec<char>; 9] {
    // Can't use the short-hand notation here because the elements
    // need to be distinct.
    let mut stacks = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let path = Path::new("crate-config.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            // Go through characters
            let mut chars = line.chars();
            // Skip the first column
            chars.next();
            // Then create a new iterator that operates in steps of 4
            let mut crate_i = chars.step_by(4);
            let mut i = 0;
            for c in crate_i {
                if c != ' ' {
                    stacks[i].insert(0, c);
                }
                i += 1;
            }
        }
    }

    return stacks;
}

fn main() {

    let mut stacks = crate_config();

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Example line:
    // move 10 from 8 to 1
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Could not compile regex");

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            // Parse line with regex
            if let Some(cap) = re.captures(&line) {
                let mut count = cap[1].parse::<usize>().expect("Could not parse crate count");
                let from = cap[2].parse::<usize>().expect("Could not parse stack index");
                let to = cap[3].parse::<usize>().expect("Could not parse stack index");

                let mut from_stack = &mut stacks[from - 1];
                let stack_height = from_stack.len();
                let mut moving = from_stack.split_off(stack_height - count);

                let mut to_stack = &mut stacks[to - 1];
                to_stack.append(&mut moving);
            }
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }

}
