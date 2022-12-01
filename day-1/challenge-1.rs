// The input is a text file with multiple blocks of numbers.
// The blocks are separated by blank lines
// Each line in a block contains one number
//
// Approach: Read through the input file line by line.
//
// If we encounter a non-blank line, parse the line as an integer and add it to
// the current accumulated score.
//
// If we encounter a blank line *or the end of the file*, compare the current
// accumulated score to the current maximum, and update the maximum if the
// current accumulated score is larger. Then reset the current accumulated score
// to 0.
//
// The maximum and current accumulated score are initialized to 0.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


fn main() {
    let mut maximum = 0;
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
                if current_accumulated_score > maximum {
                    maximum = current_accumulated_score;
                }
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
    if current_accumulated_score > maximum {
        maximum = current_accumulated_score;
    }

    // Print out the maximum
    println!("The maximum calorie count is {}", maximum);

}
