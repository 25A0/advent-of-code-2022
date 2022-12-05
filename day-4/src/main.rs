// The input is an encoded list of rucksack contents.
//
// Approach:
// Read the file line by line. Parse the four numbers with regex.
// Check whether one range is included in the other range, and
// increment the counter by one if so.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.from >= other.from && self.from <= other.to ||
        other.from >= self.from && other.from <= self.to
    }
}



fn main() {
    let mut contains = 0;
    let mut overlaps = 0;

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Example line:
    // 10-22,45-66
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").expect("Could not compile regex");

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            // Parse line with regex
            if let Some(cap) = re.captures(&line) {
                let first_range = Range {
                    from: cap[1].parse::<u32>().expect("Could not parse from match"),
                    to: cap[2].parse::<u32>().expect("Could not parse from match"),
                };
                let second_range = Range {
                    from: cap[3].parse::<u32>().expect("Could not parse from match"),
                    to: cap[4].parse::<u32>().expect("Could not parse from match"),
                };
                if first_range.contains(&second_range) || second_range.contains(&first_range) {
                    contains += 1;
                }
                if first_range.overlaps(&second_range) || second_range.contains(&first_range) {
                    overlaps += 1;
                }
            }
        }
    }

    // Print out the total score
    println!("One contains the other: {}", contains);
    println!("Overlaps: {}", overlaps);

}
