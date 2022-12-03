// The input is an encoded list of rucksack contents.
//
// Approach:
// Accumulate three lines. Create sets from the items in those
// lines, and again compute the set intersection of these items.
// The one intersecting item must be the badge.
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


fn as_set(s: &str) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    return set;
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
    let mut reader = io::BufReader::new(file).lines();

    loop {
        if let Some(line) = reader.next() {
            // This is the first line, and we assume that there are
            // two more lines after that one
            let line_1 = line.unwrap();
            let line_2 = reader.next().unwrap().unwrap();
            let line_3 = reader.next().unwrap().unwrap();

            // Convert those lines to sets
            let set_1 = as_set(&line_1);
            let set_2 = as_set(&line_2);
            let set_3 = as_set(&line_3);

            // We can't intersect all three sets in a single
            // operation, so we create an intermediate set
            // to which we add the intersection of the first
            // two rucksacks.
            let mut intermediate_set: HashSet<char> = HashSet::new();

            for c in set_1.intersection(&set_2) {
                intermediate_set.insert(*c);
            }

            // Finally, compute the intersection between the intermediate
            // set and the third set
            match intermediate_set.intersection(&set_3).next() {
                Some(item) => total_score += as_priority(*item),
                None => panic!("The lines didn't have overlapping items"),
            }

        } else {
            // We reached the end of the file
            break;
        }
    }

    // Print out the total score
    println!("The total score is {}", total_score);

}
