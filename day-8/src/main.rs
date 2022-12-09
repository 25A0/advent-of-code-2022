// The input file contains the signal as a series of characters.
//
// Approach:
// Store the tree map as a 2d-array of u8s.
// Iterate through each row and keep track of the
// maximum number we saw. Count the tree if it is higher
// than the current maximum. Then increment the maximum
// repeat for right-to left.
// Repeat for up-to down and down-to-up.

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;


struct Map {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

#[derive(Clone,Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Map {

    fn get(self: &Self, x: usize, y: usize) -> u32 {
        let index = y * self.width + x;
        return self.data[index];
    }

    fn count_visible_trees(self: &Self, mut start: Coord,
                           next_major: fn(&Self, &mut Coord) -> bool,
                           next_minor: fn(&Self, &mut Coord) -> bool,
                           reset_minor: fn(&Self, &mut Coord)
    ) -> usize {
        // Running count of visible trees
        let mut count = 0;
        // Current coordinate that we're checking
        let mut coord = start;
        // Loop through major iterations
        loop {
            // Maximum height we've seen so far in this major iteration
            let mut max_height = 0;
            // Loop through minor iterations
            loop {
                let height = self.get(coord.x, coord.y);
                if height > max_height {
                    count += 1;
                    max_height = height;
                }
                // Update the minor coord
                if !next_minor(self, &mut coord) {
                    break;
                };
            }
            // Update the major coord
            if !next_major(self, &mut coord) {
                break;
            };
            // Reset the minor coord
            reset_minor(self, &mut coord);
        }
        return count;
    }

    fn row_iterator(self: &Self, coord: &mut Coord) -> bool {
        coord.y += 1;
        coord.y < self.height
    }

    fn col_iterator(self: &Self, coord: &mut Coord) -> bool {
        coord.x += 1;
        coord.x < self.width
    }

    fn row_iterator_rev(self: &Self, coord: &mut Coord) -> bool {
        if coord.y == 0 {
            return false;
        }
        coord.y -= 1;
        true
    }

    fn col_iterator_rev(self: &Self, coord: &mut Coord) -> bool {
        if coord.x == 0 {
            return false;
        }
        coord.x -= 1;
        true
    }

    fn reset_row(self: &Self, coord: &mut Coord) {
        coord.y = 0;
    }

    fn reset_col(self: &Self, coord: &mut Coord) {
        coord.x = 0;
    }

    fn reset_row_rev(self: &Self, coord: &mut Coord) {
        coord.y = self.height - 1;
    }

    fn reset_col_rev(self: &Self, coord: &mut Coord) {
        coord.x = self.width - 1;
    }
}

fn main() {

    let mut map = Map {
        data: Vec::new(),
        width: 0,
        height: 0,
    };

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {

            // Adjust the map size
            map.height += 1;
            if line.len() > map.width {
                map.width = line.len();
            }

            for c in line.chars() {
                // Try to parse the char as an integer
                match c.to_digit(10) {
                    Some(num) => map.data.push(num + 1),
                    None => println!("Could not parse {} as integer", c),
                };
            }
        }
    }

    // Count visible trees by row, ltr
    let visible_from_left = map.count_visible_trees(Coord { x: 0, y:0 },
                                                    Map::row_iterator,
                                                    Map::col_iterator,
                                                    Map::reset_col,
    );

    // Count visible trees by row, rtl
    let visible_from_right = map.count_visible_trees(Coord { x: map.width - 1, y:0 },
                                                    Map::row_iterator,
                                                    Map::col_iterator_rev,
                                                    Map::reset_col_rev,
    );

    // Count visible trees by col, ttb
    let visible_from_top = map.count_visible_trees(Coord { x: 0, y:0 },
                                                    Map::col_iterator,
                                                    Map::row_iterator,
                                                    Map::reset_row,
    );

    // Count visible trees by col, btt
    let visible_from_bottom = map.count_visible_trees(Coord { x: 0, y:0 },
                                                    Map::col_iterator,
                                                    Map::row_iterator_rev,
                                                    Map::reset_row_rev,
    );

    let total = 0 +
        visible_from_left +
        visible_from_right +
        visible_from_top +
        visible_from_bottom;

    println!("Visible: {}", total);
}
