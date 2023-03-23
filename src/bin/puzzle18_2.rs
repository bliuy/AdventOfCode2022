use std::{collections::HashSet, io::BufRead};

const REGEX_PATTERN: &str = "([0-9]+),([0-9]+),([0-9]+)";
const GRID_UPPER_LIMIT: usize = 25;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum BlockType {
    Air,
    Rock,
    Water,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord((u16, u16, u16));

impl From<Coord> for usize {
    fn from(value: Coord) -> Self {
        let (x, y, z) = (value.0 .0, value.0 .1, value.0 .2);
        let result = (x * GRID_UPPER_LIMIT as u16 * GRID_UPPER_LIMIT as u16)
            + (y * GRID_UPPER_LIMIT as u16)
            + z;
        result as usize
    }
}

impl From<usize> for Coord {
    fn from(value: usize) -> Self {
        let x = value.div_euclid(GRID_UPPER_LIMIT * GRID_UPPER_LIMIT);
        let y = (value - (x * GRID_UPPER_LIMIT * GRID_UPPER_LIMIT)).div_euclid(GRID_UPPER_LIMIT);
        let z = (value - (x * GRID_UPPER_LIMIT * GRID_UPPER_LIMIT)).rem_euclid(GRID_UPPER_LIMIT);
        let result = Coord((x as u16, y as u16, z as u16));
        result
    }
}

impl Coord {
    fn get_left_block(&self) -> Option<Self> {
        let (x, y, z) = (self.0 .0, self.0 .1, self.0 .2);
        if x == 0 {
            None
        } else {
            Some(Coord((x - 1, y, z)))
        }
    }
    fn get_right_block(&self) -> Option<Self> {
        let (x, y, z) = (self.0 .0, self.0 .1, self.0 .2);
        if x + 1 == GRID_UPPER_LIMIT as u16 {
            None
        } else {
            Some(Coord((x + 1, y, z)))
        }
    }
    fn get_lower_block(&self) -> Option<Self> {
        let (x, y, z) = (self.0 .0, self.0 .1, self.0 .2);
        if y == 0 {
            None
        } else {
            Some(Coord((x, y - 1, z)))
        }
    }
    fn get_upper_block(&self) -> Option<Self> {
        let (x, y, z) = (self.0 .0, self.0 .1, self.0 .2);
        if y + 1 == GRID_UPPER_LIMIT as u16 {
            None
        } else {
            Some(Coord((x, y + 1, z)))
        }
    }
    fn get_behind_block(&self) -> Option<Self> {
        let (x, y, z) = (self.0 .0, self.0 .1, self.0 .2);
        if z == 0 {
            None
        } else {
            Some(Coord((x, y, z - 1)))
        }
    }
    fn get_front_block(&self) -> Option<Self> {
        let (x, y, z) = (self.0 .0, self.0 .1, self.0 .2);
        if z + 1 == GRID_UPPER_LIMIT as u16 {
            None
        } else {
            Some(Coord((x, y, z + 1)))
        }
    }

    fn get_surrounding_rocks(&self) -> Vec<Coord> {
        let mut result = Vec::new();
        if let Some(c) = self.get_left_block() {
            result.push(c);
        }
        if let Some(c) = self.get_right_block() {
            result.push(c);
        }
        if let Some(c) = self.get_upper_block() {
            result.push(c);
        }
        if let Some(c) = self.get_lower_block() {
            result.push(c);
        }
        if let Some(c) = self.get_front_block() {
            result.push(c);
        }
        if let Some(c) = self.get_behind_block() {
            result.push(c);
        }
        result
    }

    fn get_surrounding_rock_count(&self, grid: &mut Grid) -> u16 {
        let result = self
            .get_surrounding_rocks()
            .into_iter()
            .map(|c| {
                if let Some(nei) = grid.get_mut_block(c) {
                    match nei {
                        BlockType::Air => 0,
                        BlockType::Rock => 1,
                        BlockType::Water => 0,
                    }
                } else {
                    0
                }
            })
            .sum();
        result
    }
}

// NOTE: Maximum point in the input is 20, so a 25x25x25 grid should do the trick.
struct Grid {
    _grid: Vec<BlockType>,
    div: usize,
}

impl Grid {
    fn new() -> Self {
        let grid = vec![BlockType::Air; GRID_UPPER_LIMIT * GRID_UPPER_LIMIT * GRID_UPPER_LIMIT];
        Grid {
            _grid: grid,
            div: GRID_UPPER_LIMIT,
        }
    }

    fn get_mut_block(&mut self, coord: Coord) -> Option<&mut BlockType> {
        let arr_idx: usize = coord.into();
        let result = self._grid.get_mut(arr_idx);
        result
    }

    fn get_grid_length(&self) -> usize {
        self._grid.len()
    }
}

fn main() {
    // Reading inputs
    let file = std::fs::File::open("inputs/input18.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Compiling the regex pattern
    let pattern = regex::Regex::new(REGEX_PATTERN).unwrap();
    let mut rock_coords = Vec::new();

    // Getting the coords for each of the rock blocks
    for rline in reader.lines() {
        let line = rline.unwrap();
        let matched = pattern.captures(&line).unwrap();

        let x: u16 = matched.get(1).unwrap().as_str().parse().unwrap();
        let y: u16 = matched.get(2).unwrap().as_str().parse().unwrap();
        let z: u16 = matched.get(3).unwrap().as_str().parse().unwrap();

        rock_coords.push(Coord((x, y, z)));
    }

    // Spawning the grid
    let mut grid = Grid::new();

    // Populating the grid with rock blocks
    for rc in rock_coords.iter() {
        match grid.get_mut_block(*rc) {
            Some(block) => *block = BlockType::Rock,
            None => {
                unreachable!()
            }
        };
    }

    // Simulating water filling the blocks
    // BFS method used for filling simulation
    let mut processed = HashSet::new();
    let mut neighbours = std::collections::VecDeque::new();
    neighbours.push_back(Coord((0, 0, 0)));

    while let Some(c) = neighbours.pop_front() {
        // Skip processing this block if it has already been processed
        if processed.contains(&c) {
            continue;
        }

        // Converting Air to Water
        match grid.get_mut_block(c) {
            Some(block) => match block {
                BlockType::Air => {
                    *block = BlockType::Water;
                }
                BlockType::Rock => {
                    continue;
                }
                BlockType::Water => {}
            },
            None => unreachable!(),
        }

        // Marking the current block as processed
        processed.insert(c);

        // Adding the neighbours of the current block for processing
        neighbours.extend(c.get_surrounding_rocks());
    }

    // Getting the number of non-facing rock surfaces for each rock block
    let mut surface_count = 0;
    for rc in rock_coords.iter() {
        let rock_surface_count = rc.get_surrounding_rock_count(&mut grid);
        surface_count += 6 - rock_surface_count;
    }

    // Getting the number of rock facing surfaces for the remaining air blocks
    for i in 0..grid.get_grid_length() {
        match grid.get_mut_block(i.into()) {
            Some(block) => match block {
                BlockType::Air => {
                    let coord: Coord = i.into();
                    surface_count -= coord.get_surrounding_rock_count(&mut grid);
                }
                BlockType::Rock => {}
                BlockType::Water => {}
            },
            None => unreachable!(),
        }
    }

    // Printing the results
    println!("Result: {}", surface_count);
}
