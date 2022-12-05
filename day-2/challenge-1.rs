// The input is a strategy guide.
// The first challenge is simply about mapping the input
// to distinct outcomes, scoring them, and summing up that
// score.
//
// Approach:
// Read the file line by line. Parse each line by extracting
// the first and third letter, mapping that to two distinct
// moves, calculating the score of that move, and adding that
// score to the running total score.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::cmp::Ordering;

enum Shape {
    Rock,
    Paper,
    Scissor,
}

// Define how shapes are ordered. I'm not using Rust's Ord here
// because I believe that's only meant for transitive ordering,
// which isn't the case for rock-paper-scissor.
fn cmp(our_shape: Shape, their_shape: Shape) -> Ordering {
    match our_shape {
        Shape::Rock => match their_shape {
            Shape::Scissor => Ordering::Greater,
            Shape::Paper => Ordering::Less,
            Shape::Rock => Ordering::Equal,
        },
        Shape::Paper => match their_shape {
            Shape::Rock => Ordering::Greater,
            Shape::Scissor => Ordering::Less,
            Shape::Paper => Ordering::Equal,
        },
        Shape::Scissor => match their_shape {
            Shape::Paper => Ordering::Greater,
            Shape::Rock => Ordering::Less,
            Shape::Scissor => Ordering::Equal,
        },
    }
}


fn score_round(their_move: Shape, our_move: Shape) -> u32 {
    let shape_score = match our_move {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };

    let round_score = match cmp(our_move, their_move) {
        Ordering::Greater => 6,
        Ordering::Equal => 3,
        Ordering::Less => 0,
    };

    return shape_score + round_score;
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
            // Extract the letters
            assert!(line.len() == 3);
            let mut chars = line.chars();
            let their_letter = chars.next().unwrap();
            // Skip the space
            chars.next();
            let our_letter = chars.next().unwrap();

            // Map to moves
            let their_move = match their_letter {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissor,
                c => panic!("Found unexpected letter {}", c),
            };

            let our_move = match our_letter {
                'X' => Shape::Rock,
                'Y' => Shape::Paper,
                'Z' => Shape::Scissor,
                c => panic!("Found unexpected letter {}", c),
            };


            // Score round
            total_score += score_round(their_move, our_move);
        }
    }

    // Print out the total score
    println!("The total score is {}", total_score);

}
