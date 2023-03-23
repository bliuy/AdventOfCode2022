use std::{io::BufRead, ops::Mul};

struct Circuit {
    cycle: i32,
    register: i32,
    signal: i32,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            cycle: 0,
            register: 1,
            signal: 0,
        }
    }

    fn cycle_callback(&mut self) {
        println!("Cycle: {} - Register: {}", self.cycle, self.register);
        if self.cycle > 220 {
            return;
        }

        if (self.cycle == 20) | ((self.cycle - 20).rem_euclid(40) == 0) {
            let signal = self.cycle.mul(self.register);
            self.signal += signal;
            // println!("{} * {} - {}", self.cycle, self.register, signal);
        }
    }

    fn noop(&mut self) {
        self.cycle += 1; // Starting a new cycle.
        self.cycle_callback() // Callback during the current cycle.
    }

    fn add(&mut self, val: i32) {
        self.noop(); // 1st cycle
        self.noop(); // 2nd cycle
        self.register += val; // Incrementing the register AFTER the 2nd addx cycle completes.
    }
}

fn main() {
    // Reading the file
    let file = std::fs::File::open("inputs/input10.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);
    let mut circuit = Circuit::new();

    for rline in reader.lines() {
        let line = rline.unwrap();
        let pattern = regex::Regex::new("([A-z]{4}) ?(-*[0-9]*)").unwrap();

        let captures = pattern.captures(&line).unwrap();
        let op = captures.get(1).unwrap().as_str();
        let possible_val = captures.get(2);
        match op {
            "noop" => {
                circuit.noop();
            }
            "addx" => {
                let val = possible_val.unwrap().as_str().parse::<i32>().unwrap();
                circuit.add(val);
            }
            _ => panic!(),
        }
    }

    println!("Result: {}", circuit.signal);
}
