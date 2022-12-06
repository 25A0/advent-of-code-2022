// The input file contains the signal as a series of characters.
//
// Approach:
// Read characters into a vector. When the vector contains five elements,
// remove the first element. Then create a hashset from the remaining characters,
// and check if it has four elements. If it does, then we've reached the target
// position and we return the number of characters that we've read so far.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn is_start_of_packet_marker(ring_buf: &Vec<char>) -> bool {
    let mut set = HashSet::new();
    for c in ring_buf {
        set.insert(c);
    }
    return set.len() == 4;
}

fn main() {

    let mut ring_buf = Vec::new();

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let mut n_chars = 0;
            for c in line.chars() {
                n_chars += 1;
                ring_buf.push(c);
                if ring_buf.len() > 4 {
                    // Remove the first elements
                    ring_buf.remove(0);
                    if is_start_of_packet_marker(&ring_buf) {
                        println!("Start of packet marker ends at char {}", n_chars);
                        break;
                    }
                }
            }
        }
    }

}
