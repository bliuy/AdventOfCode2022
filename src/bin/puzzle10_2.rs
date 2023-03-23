use std::{
    io::BufRead,
    ops::{Mul, Range, RangeInclusive},
};

struct Circuit {
    cycle: i32,
    pixels: Vec<char>,
    sprite: Sprite,
}

struct Sprite {
    register: i32,
    pos: RangeInclusive<usize>,
}

impl Sprite {
    fn new() -> Self {
        Sprite {
            register: 1,
            pos: 0..=1,
        }
    }

    fn update(&mut self, steps: i32) {
        // Updating the register
        self.register += steps;

        // Updating the position of the sprite
        let mut start = self.register - 1;
        let mut end = self.register + 1;

        // Maintain the bounds of the sprite
        if start < 0 {
            start = 0;
        }

        if end > 39 {
            end = 39
        }

        // Updating the internal pos field
        self.pos = start as usize..=end as usize;
    }
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            cycle: 0,
            pixels: Vec::new(),
            sprite: Sprite::new(),
        }
    }

    fn cycle_callback(&mut self) {
        // Getting the current pixel position
        let pixel_pos = self.cycle - 1;

        // Getting the horizontal pixel position
        let horizontal_pixel_pos = pixel_pos.rem_euclid(40) as usize;

        // Rendering the correct character based on whether overlapping or not
        println!(
            "{} against the range of {:?}",
            &horizontal_pixel_pos, &self.sprite.pos
        );
        if self.sprite.pos.contains(&horizontal_pixel_pos) {
            self.pixels.push('#');
        } else {
            self.pixels.push('.');
        }
    }

    fn noop(&mut self) {
        self.cycle += 1; // Starting a new cycle.
        self.cycle_callback() // Callback during the current cycle.
    }

    fn add(&mut self, val: i32) {
        self.noop(); // 1st cycle
        self.noop(); // 2nd cycle
        self.sprite.update(val);
    }

    fn print_display(&self) {
        for chunk in self.pixels.chunks(40) {
            println!("{:?}", chunk);
        }
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

    circuit.print_display();
}
