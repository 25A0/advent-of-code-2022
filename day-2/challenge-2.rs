// In the second challenge, we need to find the right counter-move
// for each of the opponent's moves.
//
// Approach:
// Read the file line by line. Parse the opponent's move and map
// it to a shape, then parse the desired outcome, and find the right
// counter-move for that. Finally, score the round as before
// and compute the sum of all scores.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
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

// Define the right couter-move for the opponent's shape
// and the desired outcome.
fn find_our_shape(outcome: Ordering, their_shape: Shape) -> Shape {
    match outcome {
        // We should win
        Ordering::Greater => match their_shape {
            Shape::Scissor => Shape::Rock,
            Shape::Paper => Shape::Scissor,
            Shape::Rock => Shape::Paper,
        },
        // They should win
        Ordering::Less => match their_shape {
            Shape::Rock => Shape::Scissor,
            Shape::Scissor => Shape::Paper,
            Shape::Paper => Shape::Rock,
        },
        // The round should be a draw. We're returning a clone here because it's
        // not supposed to be the same instance of the enum, just the same shape
        Ordering::Equal => their_shape.clone(),
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
            let outcome = chars.next().unwrap();

            // Map to moves
            let their_move = match their_letter {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissor,
                c => panic!("Found unexpected letter {}", c),
            };

            // Map to desired outcome. We express that with an ordering
            // that compares our move to their move.
            // it can be read as our_move < / = / > their_move
            let outcome_ordering = match outcome {
                'X' => Ordering::Less,
                'Y' => Ordering::Equal,
                'Z' => Ordering::Greater,
                c => panic!("Found unexpected letter {}", c),
            };

            let our_move = find_our_shape(outcome_ordering, their_move);


            // Score round
            total_score += score_round(their_move, our_move);
        }
    }

    // Print out the total score
    println!("The total score is {}", total_score);

}
