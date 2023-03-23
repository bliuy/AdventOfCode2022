use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    // Reading the input
    let file = File::open("inputs/input4.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);

    // Compiling the regex used for matching the pattern
    let pattern = Regex::new("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();

    let mut total = 0;
    // Processing each group
    for line in reader.lines() {
        let line = line.unwrap(); // Should always be able to read the line.
        let captures = pattern.captures(&line).unwrap(); // Should always be able to capture the intended groups.
        let (a, b, c, d) = (
            captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        );

        if (a <= c) & (b >= d) {
            total += 1;
            println!("({},{}) covers ({},{})", a, b, c, d);
        } else if (c <= a) & (d >= b) {
            total += 1;
            println!("({},{}) covers ({},{})", c, d, a, b);
        }
    }
    println!("Result: {}", total);
}
