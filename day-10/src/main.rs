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

    let mut signal_sum: isize = 0;
    for i in 0..6 {
        let cycle = 20 + i * 40;
        //  Read signal at cycle - 1, since the vector indexing starts at 0
        let signal = signal_series[cycle - 1];
        signal_sum += (cycle as isize) * signal;
    }

    println!("Signal sum is {}", signal_sum);
}
