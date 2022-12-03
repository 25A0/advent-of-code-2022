// The input is an encoded list of rucksack contents.
//
// Approach:
// Read the file line by line. Split the line in the middle
// to get items in each of the two compartments. Then turn
// each half into a set of characters, and compute the set
// intersection of both compartments. The intersection
// should be the single item that is in both compartments.
//
// Map that item to a priority and add it to the running sum.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn as_priority(letter: char) -> u32 {
    if letter.is_uppercase() {
        return (letter as u32) - ('A' as u32) + 27;
    } else {
        return (letter as u32) - ('a' as u32) + 1;
    }
}


fn main() {
    let mut total_score = 0;

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let len = line.len();
            assert!(len % 2 == 0);

            let first_half: &str = &line[..len/2];
            let second_half: &str = &line[len/2..];

            // Items in the first compartment
            let mut first: HashSet<char> = HashSet::new();
            for c in first_half.chars() {
                first.insert(c);
            }
            // Items in the second compartment
            let mut second: HashSet<char> = HashSet::new();
            for c in second_half.chars() {
                second.insert(c);
            }

            // Find items in both compartments
            match first.intersection(&second).next() {
                Some(item) => total_score += as_priority(*item),
                None => panic!("Line {} didn't have overlapping items", line),
            }
        }
    }

    // Print out the total score
    println!("The total score is {}", total_score);

}
