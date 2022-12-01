// The input is a text file with multiple blocks of numbers.
// The blocks are separated by blank lines
// Each line in a block contains one number
//
// In this challenge we need to first sum up each block,
// and finally sum up the three largest block.
//
// Approach: First sum up the blocks as before, but instead
// of keeping track of a single maximum, we assemble them
// all into a vector, sort that vector, and sum up the
// first three elements.
//

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


fn main() {
    let mut blocks: Vec<u32> = Vec::new();
    let mut current_accumulated_score = 0;

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.trim().len() == 0 {
                // We reached the end of a block.
                // Add the current score to the blocks
                blocks.push(current_accumulated_score);
                current_accumulated_score = 0;
            } else {
                // Try to parse the line as an integer
                match line.trim().parse::<u32>() {
                    Ok(num) => current_accumulated_score += num,
                    Err(_) => println!("Could not parse {} as integer", line),
                };
            }
        }
    }

    // We reached the end of the file
    blocks.push(current_accumulated_score);

    // Now sort that vector, and reverse it so that the largest elements are at
    // the start of the vector
    blocks.sort_unstable();
    blocks.reverse();

    if blocks.len() >= 3 {
        println!("The last three elements are {:?}", &blocks[..3]);
        let mut sum = 0;
        for i in &blocks[..3] {
            sum += i;
        }
        println!("The sum of those three elements is {}", sum);
    }

}
