// The input is a series of CPU instructions


use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {

    // This vector contains the register value during cycle i+1 at index i.
    let mut signal_series = Vec::new();

    // The value of the register
    let mut reg = 1;

    // The register is 1 initially
    signal_series.push(reg);

    let path = Path::new("input.txt");

    // Open the file for reading
    let file = match File::open(&path) {
        Err(e) => panic!("Couldn't open input file: {}", e),
        Ok(file) => file,
    };

    // Example line:
    // noop
    // addx -4
    let re = Regex::new(r"addx (-?\d+)").expect("Could not compile regex");

    // Iterate through the lines in the file
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line == "noop" {
                // Nothing changes this cycle
                signal_series.push(reg);
            } else if let Some(cap) = re.captures(&line) {
                let val = &cap[1].parse::<isize>().expect("Could not parse addx value");
                // The addx instruction takes two cycles to complete,
                // so nothing changes in the first cycle
                signal_series.push(reg);
                // After that cycle, the register is updated
                reg += val;
                signal_series.push(reg);
            } else {
                panic!("Line {} didn't match any known instruction", line);
            }

        }
    }

    // Loop over lines
    for line in 0..6 {
        // Loop over pixels
        for pixel in 0..40 {
            let cycle = 1 + line * 40 + pixel;
            let reg_val = signal_series[cycle - 1];
            if (reg_val - pixel as isize).abs() <= 1 {
                // The sprite is in range, so we draw a #
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
