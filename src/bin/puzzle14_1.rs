use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::BufRead,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: u32,
    y: u32,
}

impl PartialOrd for Coords {
    // In this case - The main ordering is based on the lowest point on the y-axis.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for Coords {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Rock {
    loc: Coords,
}

impl Rock {
    fn new(loc: Coords) -> Self {
        Rock { loc: loc }
    }

    fn construct(start: Coords, end: Coords) -> Vec<Rock> {
        match start.x == end.x {
            true => {
                let mut result = Vec::new();
                let min: u32;
                let max: u32;
                match start.y < end.y {
                    true => {
                        min = start.y;
                        max = end.y;
                    }
                    false => {
                        min = end.y;
                        max = start.y;
                    }
                }
                let mut current = min;
                while current <= max {
                    let coords = Coords::new(start.x, current);
                    result.push(Rock::new(coords));
                    current += 1;
                }
                result
            }
            false => {
                let mut result = Vec::new();
                let min: u32;
                let max: u32;
                match start.x < end.x {
                    true => {
                        min = start.x;
                        max = end.x;
                    }
                    false => {
                        min = end.x;
                        max = start.x;
                    }
                }
                let mut current = min;
                while current <= max {
                    let coords = Coords::new(current, start.y);
                    result.push(Rock::new(coords));
                    current += 1;
                }
                result
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Sand {
    loc: Coords,
}

impl Sand {
    fn new() -> Self {
        let coords = Coords::new(500, 0);
        Sand { loc: coords }
    }
}

trait Move {
    fn moved(&mut self, rocks: &[Rock], sands: &mut HashSet<Sand>, lowest: Coords) -> AtRest;
}

type AtRest = bool;

impl Move for Sand {
    fn moved(&mut self, rocks: &[Rock], sands: &mut HashSet<Sand>, lowest: Coords) -> AtRest {
        let rock_locations = rocks
            .into_iter()
            .map(|rock| rock.loc)
            .collect::<HashSet<_>>();
        loop {
            // Checking to see if it has fallen off the abyss
            match self.loc.y > lowest.y {
                true => return false,
                false => {
                    // Attempting to move downwards
                    self.loc.y += 1;
                    match (rock_locations.contains(&self.loc)) | (sands.contains(&self)) {
                        true => {
                            // Attempt to move downwards-left
                            self.loc.x = self.loc.x - 1;
                            match (rock_locations.contains(&self.loc)) | (sands.contains(&self)) {
                                true => {
                                    // Attempting to move downwards right
                                    self.loc.x = self.loc.x + 2;
                                    match (rock_locations.contains(&self.loc))
                                        | (sands.contains(&self))
                                    {
                                        true => {
                                            // Blocked in moving from all directions - Particle comes to rest
                                            self.loc.x = self.loc.x - 1;
                                            self.loc.y = self.loc.y - 1;
                                            sands.insert(*self);
                                            return true;
                                        }
                                        false => {
                                            continue; // Sand not blocked from falling in downwards-right direction.
                                        }
                                    }
                                }
                                false => {
                                    continue; // Sand not blocked from falling in downwards-left direction.
                                }
                            }
                        }
                        false => {
                            continue; // Sand not blocked from falling downwards
                        }
                    }
                }
            }
        }
    }
}

impl Coords {
    fn new(x: u32, y: u32) -> Self {
        Coords { x: x, y: y }
    }

    fn from_str(input: &str) -> Self {
        let mut split = input.split(",");
        let x: u32 = match split.next() {
            Some(_a) => match _a.parse() {
                Ok(_b) => _b,
                Err(_) => unreachable!(),
            },
            None => unreachable!(),
        }; // Standardized format, getting the element at the 0th index.
        let y: u32 = match split.next() {
            Some(_a) => match _a.parse() {
                Ok(_b) => _b,
                Err(_) => unreachable!(),
            },
            None => unreachable!(),
        }; // Standardized format, getting the element at the 1st index.
        Coords { x: x, y: y }
    }
}

fn main() {
    let file = std::fs::File::open("inputs/alt/input14.txt").expect("Failed to open file.");
    // let file = std::fs::File::open("inputs/examples/example14.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Start constructing the locations of the rocks
    let mut rocks = HashSet::new();
    for rline in reader.lines() {
        let line = match rline {
            Ok(_line) => _line,
            Err(_) => unreachable!(),
        };
        rocks.extend(
            line.split(" -> ")
                .into_iter()
                .map(|x| Coords::from_str(x))
                .collect::<Vec<_>>()
                .windows(2)
                .map(|window| {
                    let first = match window.first() {
                        Some(x) => x,
                        None => unreachable!(),
                    };
                    let second = match window.last() {
                        Some(x) => x,
                        None => unreachable!(),
                    };
                    Rock::construct(*first, *second)
                })
                .fold(Vec::new(), |mut accumulated, rock| {
                    accumulated.extend(rock.into_iter());
                    accumulated
                })
                .into_iter(),
        );
    }

    // Getting the lowest point
    let lowest = match rocks.iter().map(|x| x.loc).max() {
        Some(x) => x,
        None => unreachable!(),
    };

    // Start dropping the sand particles
    let mut sands = HashSet::new();
    let mut count = 0;
    let rocks_arr = rocks.into_iter().collect::<Vec<_>>();
    loop {
        let mut sand = Sand::new();
        match sand.moved(&rocks_arr, &mut sands, lowest) {
            true => {
                count += 1;
            }
            false => {
                println!("Result: {}", count);
                break;
            }
        };
    }
}
