use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Coords { x: x, y: y }
    }
}

type AtRest = bool;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Obstacle {
    Rock(Coords),
    Sand(Coords),
}

type ObstacleLocations = HashSet<Coords>;

impl Obstacle {
    fn create_sand() -> Self {
        let coords = Coords::new(0, 500);
        Self::Sand(coords)
    }
    fn create_rock(coords: Coords) -> Self {
        Self::Rock(coords)
    }
    fn dropdown(&mut self, existing: &mut ObstacleLocations, lowest: &Coords) -> AtRest {
        println!("Sand starts falling!");
        let current = match self {
            Obstacle::Rock(_) => unimplemented!(), // Rocks should never be able to move.
            Obstacle::Sand(loc) => loc,
        };

        loop {
            println!("Current location of sand: {}, {}", current.x, current.y);
            // Check - If Sand has fallen into the Abyss
            if current.x > lowest.x {
                println!("Sand falls into the abyss!");
                return false;
            }

            // Attempting to move downwards
            current.x += 1;
            if !existing.contains(&current) {
                println!("Sand falls downwards!");
                continue; // Sand is not blocked in the downwards direction.
            }

            // Attempting to move left-down
            current.y = current.y - 1;
            if !existing.contains(&current) {
                println!("Sand falls diagonally left!");
                continue; // Sand is not blocked in the downwards direction.
            }

            // Attempting to move right-down
            current.y += 2;
            if !existing.contains(&current) {
                println!("Sand falls diagonally right!");
                continue; // Sand is not blocked in the downwards direction.
            }

            // All 3 locations blocked - Sand comes to a rest
            println!(
                "Sand comes to rest at the following position: {}, {}",
                &current.x, &current.y
            );
            current.y = current.y - 1; // Resetting to original y
            current.x = current.x - 1; // Resetting to original x
            existing.insert(*current);
            return true;
        }
    }
}

fn main() {
    let file = std::fs::File::open("inputs/alt/input14.txt").expect("Failed to open file.");
    // let file = std::fs::File::open("inputs/input14.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    let mut obstacle_locations = HashSet::new();
    let mut lowest: Option<Coords> = None;

    for rline in reader.lines() {
        let line = match rline {
            Ok(v) => v,
            Err(_) => unreachable!(),
        };

        obstacle_locations.extend(line.split(" -> ").into_iter().map(|x| {
            let splitted = x.split(",").collect::<Vec<_>>();
            // Getting the x and y coordinates
            let x: usize = match splitted.last() {
                Some(v) => match v.parse() {
                    Ok(v) => v,
                    Err(_) => unreachable!("Should always be able to be parsed into usize."),
                },
                None => unreachable!("Should always be in the consistent format."),
            };
            let y: usize = match splitted.first() {
                Some(v) => match v.parse() {
                    Ok(v) => v,
                    Err(_) => unreachable!("Should always be able to be parsed into usize."),
                },
                None => unreachable!("Should always be in the consistent format."),
            };
            // Constructing the Coords object
            Coords::new(x, y)
        }));
    }

    println!("{:?}", lowest);

    let lowest_coords = match lowest {
        Some(coords) => coords,
        None => unreachable!(),
    };

    let mut counted = 0;
    loop {
        let mut sand = Obstacle::create_sand();
        match sand.dropdown(&mut obstacle_locations, &lowest_coords) {
            true => {
                counted += 1;
            }
            false => {
                println!("Result: {}", counted);
                break;
            }
        };
    }
}
