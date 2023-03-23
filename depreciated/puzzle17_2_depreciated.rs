use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::Sub,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: u64,
    y: u64,
}

type HasMoved = Option<()>;
type TopLayerPattern = u64;

struct TopLayer {
    coords: Vec<Coord>,
    highest: u64,
}

impl TopLayer {
    fn trim(&mut self) {
        let result = self
            .coords
            .iter_mut()
            .filter(|c| !(c.y + 7 < self.highest))
            .map(|c| *c)
            .collect::<Vec<Coord>>();
        self.coords = result;
    }

    fn insert_coord(&mut self, coord: Coord) {
        match coord.y > self.highest {
            true => {
                self.highest = coord.y;
                self.coords.push(coord);
                self.trim();
            }
            false => match coord.y + 7 < self.highest {
                true => {} // No need to even add into consideration
                false => {
                    self.coords.push(coord);
                    self.trim();
                }
            },
        }
    }

    fn get_pattern_id(&self) -> TopLayerPattern {
        let mut hasher = DefaultHasher::new();
        self.coords.iter().for_each(|c| {
            let mut normalized_coord = c.clone();
            normalized_coord.y = normalized_coord.y.rem_euclid(8);
            normalized_coord.hash(&mut hasher);
        });

        let result = hasher.finish();
        result
    }
}

impl Coord {
    fn new(x: u64, y: u64) -> Self {
        Coord { x: x, y: y }
    }

    fn displace_left(&self) -> Option<Coord> {
        match self.x.checked_sub(1) {
            Some(x_new) => match x_new {
                0 => None,
                _ => Some(Coord::new(x_new, self.y)),
            },
            None => None,
        }
    }
    fn displace_right(&self) -> Option<Coord> {
        match self.x.checked_add(1) {
            Some(x_new) => match x_new {
                8 => None,
                _ => Some(Coord::new(x_new, self.y)),
            },
            None => None,
        }
    }
    fn displace_down(&self) -> Option<Coord> {
        match self.y.checked_sub(1) {
            Some(y_new) => match y_new {
                0 => None,
                _ => Some(Coord::new(self.x, y_new)),
            },
            None => None,
        }
    }
}

enum JetDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum RockShape {
    Minus,
    Plus,
    LShaped,
    Bar,
    Square,
}

#[derive(Debug)]
struct Rock {
    coords: Vec<Coord>, // Allows for the iter_mut method that HashSet doesn't.
    shape: RockShape,
}

impl Rock {
    fn new(shape: RockShape, datum: u64) -> Self {
        let bottom_y = datum + 4;
        let left_x = 3;
        match shape {
            RockShape::Minus => {
                let mut coords = Vec::new();
                for (x, y) in vec![
                    (left_x, bottom_y),
                    (left_x + 1, bottom_y),
                    (left_x + 2, bottom_y),
                    (left_x + 3, bottom_y),
                ] {
                    let coord = Coord::new(x, y);
                    coords.push(coord);
                }

                Rock {
                    coords: coords,
                    shape: shape,
                }
            }
            RockShape::Plus => {
                let mut coords = Vec::new();
                for (x, y) in vec![
                    (left_x, bottom_y + 1),
                    (left_x + 1, bottom_y),
                    (left_x + 1, bottom_y + 1),
                    (left_x + 1, bottom_y + 2),
                    (left_x + 2, bottom_y + 1),
                ] {
                    let coord = Coord::new(x, y);
                    coords.push(coord);
                }

                Rock {
                    coords: coords,
                    shape: shape,
                }
            }
            RockShape::LShaped => {
                let mut coords = Vec::new();
                for (x, y) in vec![
                    (left_x, bottom_y),
                    (left_x + 1, bottom_y),
                    (left_x + 2, bottom_y),
                    (left_x + 2, bottom_y + 1),
                    (left_x + 2, bottom_y + 2),
                ] {
                    let coord = Coord::new(x, y);
                    coords.push(coord);
                }

                Rock {
                    coords: coords,
                    shape: shape,
                }
            }
            RockShape::Bar => {
                let mut coords = Vec::new();
                for (x, y) in vec![
                    (left_x, bottom_y),
                    (left_x, bottom_y + 1),
                    (left_x, bottom_y + 2),
                    (left_x, bottom_y + 3),
                ] {
                    let coord = Coord::new(x, y);
                    coords.push(coord);
                }

                Rock {
                    coords: coords,
                    shape: shape,
                }
            }
            RockShape::Square => {
                let mut coords = Vec::new();
                for (x, y) in vec![
                    (left_x, bottom_y),
                    (left_x, bottom_y + 1),
                    (left_x + 1, bottom_y + 1),
                    (left_x + 1, bottom_y),
                ] {
                    let coord = Coord::new(x, y);
                    coords.push(coord);
                }

                Rock {
                    coords: coords,
                    shape: shape,
                }
            }
        }
    }

    fn pushed_by_jet(
        &mut self,
        jet_direction: JetDirection,
        existing: &HashSet<Coord>,
    ) -> HasMoved {
        // Generating the moved positions
        let potential_positions = match self
            .coords
            .iter()
            .map(|c| match jet_direction {
                JetDirection::Left => c.displace_left(),
                JetDirection::Right => c.displace_right(),
            })
            .collect::<Option<Vec<_>>>()
        {
            Some(x) => x,
            None => return None,
        };

        // Checking for any intersecting coordinates.
        let result = potential_positions
            .iter()
            .map(|x| match existing.contains(x) {
                true => None,
                false => Some(()),
            })
            .collect::<Option<()>>();

        match result {
            Some(_) => {
                self.coords = potential_positions;
                Some(())
            }
            None => None,
        }
    }

    fn fall_downwards(&mut self, existing: &HashSet<Coord>) -> HasMoved {
        // Generating the moved positions
        // If None, indicates that negative values have occured - Rock reaches an invalid position
        let potential_positions = match self
            .coords
            .iter()
            .map(|c| c.displace_down())
            .collect::<Option<Vec<_>>>()
        {
            Some(x) => x,
            None => return None,
        };

        // Checking for any intersecting coordinates.
        // If None, indicates that the new positions of the rock will intersect with an existing position.
        let result = potential_positions
            .iter()
            .map(|x| match existing.contains(x) {
                true => None,
                false => Some(()),
            })
            .collect::<Option<()>>();

        match result {
            Some(_) => {
                self.coords = potential_positions;
                Some(())
            }
            None => None,
        }
    }

    fn get_highest_point(&self) -> u64 {
        let result = self.coords.iter().map(|c| c.y).max().unwrap();
        result
    }
}

fn main() {
    const LIMIT_COUNT: u64 = 1000000000000;

    // Configuring rock sequence
    let rock_sequence = [
        RockShape::Minus,
        RockShape::Plus,
        RockShape::LShaped,
        RockShape::Bar,
        RockShape::Square,
    ];
    let mut rock_seq_iter = rock_sequence.into_iter().cycle();
    let mut datum = 0;

    // Configuring jet stream
    let mut jet_stream_iter = ">>>><<<><>>><<<<>>><>>><>><<<<>>><>><<<<>><<<<>><<<>>>><>>>><<<<>>><>><<>>>><>>>><<>>>><<<<>>>><<<<>>><<>>><<<>>><<<<>><<>><>>>><<<>>><>><<<<>><<<>><<>>>><<<<>>>><<><>>><<<<><>><<<<>>>><><<<<>>><<<<>><<<<>>><<<>><<<>>><<>><>>>><<<>>><>>>><<<<>>>><<>>>><><>>>><<<><<<<><<<<>>><<>>><<<<>>><<<>><<<<>><>><<>>><<>>><<<<>>>><<>>><<<>>><<>>><<<>>><<<>><<>><<<>>>><>><<<<>>>><><>><>><<><<<><<<<>>><<<<>>><>><<<<><<<>>><<<<>><<>><<<<>>><<<>>><<>>>><<<>>><<<>>><<<<><<>><>><<<<><<<><<><<<><<<>>>><<<><<>>><<<<><<<<>>><>><>>><<<>><<>><<<<>>><<>>><><<<<>><<><<<<>><<<<>>><<<<>><>><<>><<<<>><<>>>><<<>><<<<>>><<<>>>><><<<>>>><<<>>>><<><<<<>>>><><<>>>><<>><<<<>>>><><<<>><><<<>>><><<<>><<<>>><>>><<<>>><<<<>>>><<<>>>><>>><>>>><<>><<><<<><<>>><<>><<<><<<<><<<>>>><<>>><<>><<>><><><<>>><>>>><<>><<<>>>><>>><><<>>>><>>>><>>><<<>><<<<>><<><<<<>>><<><<>>>><<<<>>><<<<>>>><<<<>><<><<<<>><>>>><<<<>><<<>>><<<>>>><>><>>>><>><<>>><<>>>><<<>><<>>>><<<<>>>><<><<<<>>><><<<><<<<>>><<>>>><>>>><>>>><<>><<>><<><<<><<<<>>><<<<><<<>>>><<>>><<<<>>><>>>><<>>>><<>><<>>><<>><<<<>><>><<<<>>>><<>>><<<>>>><>>>><<<><>>>><<>>>><<<>>><<><>>><>>><<<<><>><<<>><<><<<>><><>><><<>><<<>><<<>><<<>>><<>><<>><<<<>><>>><><<<><>><>><<<<>><<>>><<>>>><<<<>><<<<>>>><<<<><<<<>>><>>>><<<>>>><<<>><<><<<<>>><<<><<><<<>>>><<<<>><<>>>><>>>><<>><<<<>><<<<><>>>><><<<<>><>><<>>><><>><><><<<<>>>><>>><<>><<<>>><<<>><<>>>><<<><<<<><>><<>><>><>>><>>><<>>><<<<>><>>>><>>><<<<><<><<<<>><>><>>>><>><<>>><>><><<<><<<>>><<<>>><<<<><<>>>><><<<>>>><<>>><<<>>><<<>>><<<<><><<<>>>><<<<>>>><<<<>>><><<<<><>>>><<<><>>>><>>>><<<<>>>><<<>>>><<<<><<<>><<>>><<<>><>><>>><><<<<><<<<><<<><><<<<>><<>>>><>>><><<<<>>><>><<<>><<>>><<<><<><<>>>><<<<>>><<<>>>><<<>>><<><<<><<<<>>>><<>><>><<<>>>><<>>><>>><<<>>><<<<>>><><<<<>>>><<<>><<<><<>><<>>>><<<>><<>>>><<>><<<><>>><>><><<<<>>>><><<<<>>><<<>>>><<<<>>>><<>>>><<>>><>>>><<<>>>><<<>>><<<><<<<><>><<<<>>><<<<>>><<>><<<<>>><>>><<<<>>>><<>><<>>><<<<><<>>>><<>><<>>>><>>>><<<>>>><<<><>><>>>><<<<>><<>>>><<<>>><<<<>>>><><<>>><<<>>><<<>>><<>>>><<<>>><>>><<<<>><<><<<<>><<<<>>>><<<>>>><>>>><>>><<>>>><<><<<>><>><<<>>>><<<<><<<<><<<<>>><<<<><><<>><<>><<<<>>>><<>>>><>>><<>>>><<<<><>><<>>>><>><>>><<<>><<<<>><<<><<<>>>><<<>><<>>>><<<<><<<<>>><<<>><>>><<><<>>>><<<<><<<<>>>><<<>>>><<>>><>>><<<>><<><<<>>>><>>>><<<>>>><<<<>>><<<>>>><>>>><<<<>>>><<<>><<<<>>>><><<<>><<<<><<<><<>>>><<>>><>>><<<>>><<<<><<><<><<<><<<>>>><<<<><<<<><<><<>>><>><<>><<><<<>>>><<<><<<<>>><>>>><<<<>>>><<<<><<<>>>><<<>>>><>><<>>><>>>><<>>><<>>><>><<>><<>>>><<>>>><>><<><<<<>>><<>>><>>>><<<<>>><<<>>><<<><<<<><<>><>><<<<>>><<>>>><>>>><<<><><<>>><<><<<><<<><>>>><<><>><>><>><<<>>>><<<>><<>>>><<<>>><>>>><<<><<><<<<>>>><>><<><<<<><<<><<<>><><<<>><><<<<>>><<<<>><<<<><<<>>>><<<<>>>><<<>><<><>>>><<>>><>>><<<>>><>>>><<>><<<<>><<<<>>>><<>>><<<>><<<<>>><<<<>>>><>>>><<<<>><>>>><<<>><>>><<<<>>><<<<><<<<>>><<>><<<>><<<<>><<><<>>><<>>><<<>><<>>><<>>>><<<>>>><<<<>>>><<><>>>><<>><>><<<>>><<<>>>><>>>><<<>><<><><>>>><<><<<>><>><><<>><<<>>><<>><<<<><<>>><<<><<<<>>><<>><<<>>><<>>>><<<>><<<><<<<>>>><<<>>><<<<><<<<><>>>><<<>><<<>>>><<<>>>><><<<<>>>><>>>><<<>><>><<<<>><><<<>>><<<<>>>><>><<<<>><><<<>>>><<<>>>><<<<>>><>><<<<>><<<<>><<<<>>><<>><<<><>>><><<<<>>><<<>>>><<<>>>><<<>>><<<<>><<><<<<>>><<>>>><<<<>><<>>><>>><<<>>><<<<>>>><<>><<<>><<>>>><<>>><<<<>>><<>>>><><<<>>>><<<<>>><<<<>><<<>>><>><<>>><<>>>><<<>>><<>>>><>><>>>><<<<>>>><>>>><<<>><<<<>>>><<<<>>><>>><<<>>>><<><<><><<<><<<>>>><<>>><<>><<<<>><<>>><<><<<<>><<<>>><<>><>><<><<<<><>>><<<>>>><>>>><<<<><<<>><<>>><>><>><<>>><<<><<<<>>><>>>><<<>>>><<<>>>><>><><<<>>>><<<<><<<>>>><<<<>>><>>><<<<>>><>>>><<>>><<<>><<<<><<<><<>>><<<<><<>>>><<<>>>><<<>>><<>><<<<>><><<<<><<<>><<>>>><>>><<><<><<<><<>>>><<>><<>>>><<>>>><<<>>><<>><<>>><<<><>>>><<<>>><>>><>>><>><<>><>><<<<>>>><><<<<>>><><<<>>>><<<<>><<<<>>><<>>><<<>>>><<<<>>>><<<<><>>><<><<<>>>><<<>><<<<>>>><<<><<<>><<<>><<>>><<<>>><><><>>>><>>>><<>>><<<>>><>>>><<<>><>>><>>><<<<>><<><<<<><<><<<<>>>><<<>><><>>>><<<>>><<<>><<>>>><><<<<>><<<<>>>><<<><<>>><<>>><<<<>>>><<>><<>>><>>><<>>><<><>><<>><>>><<<<>><<<>>>><<<>><<<<>>>><>>><<<>>><<<<>>>><<<>><<><>>>><<<<><<>>><<>>>><<<<><>>><<<<>>>><><<<<>>><>><<<<>><<<<><<<>>><<<<><<>>>><<<<>>><<<><<><<<>><<<<><<<>><<<><>>><<<<>>>><>>><<>>>><<<>>><<<<>>>><<<<>>>><<<>><<>>><<>>><<<>>><<<<>><<<>>><<><<>>><<<<>>>><>>><<<>><<<>>><<><<<>>>><<<><<<<>>>><<<>><<<<>>><<>>>><<>><<<><><><<<><<<>>><<<>><><><<<><<<<>><>>>><<<>>><<<>><<<>>>><>><<>><>>>><<>>>><><>><<>>>><<<>>><<<>>>><<<>>>><<<><<><<<>><>>><<<<>>>><<>><<<<><>>><<>>>><<<<>>><<><<<>>>><<<><<<>>>><<<<>>><<<<>>><<<>><<>>>><<<>>>><<<><<><>>><<<<>>><<>>><<>>><<<><<<><<>><<<><>><<>><<<<><<>>>><>>><>>><>>><>><><<<>>><<<>>>><<>><>><<><<>>><<>>><><<<><<><<<<>>><>>>><<<<>>>><<>>>><<<>><<<>><><<<<>>><<<><<<<>>><>>>><<>>>><<>><<<>>><<<><<<<>>><<>>><<><>>>><><<<>>><<<>><><><<<><<<><<<<>>><<><>>><<<<>><<<<>>><<<><<<>>><<><<<><<<><<<<>><><>><><<<>>>><<>>><>><>>><><<<<>>>><<<<>>>><<<>>>><<<>>><<<<>>>><<<<><<<<>><>>>><<><<<<>>>><<>>><<><><<<<>><<<>>>><<<>>><<<><<>>>><<><<>>><<>><<>><<<>>>><<>>><<<<>>>><<<><>>>><>><<<>>>><><<<<>>>><<>>>><<<>>><<>>>><<<>>><<<<>><<>>><<<<>>><>>><<<><<>>><>><<>><<<<><<<<>>>><<<<>><>>><<<<>>>><<<<>>>><<<<>><<<<>>><<<>>><<<>>><<>><>>><<>>>><>>><>>>><>>><<<><<<>><<<><<<<><<<>><>><><<<<>>>><<>>><<<>>>><<><>>>><<<<>>><<><<>><<><<<><<<<>><>>>><><<<><<>><<<>><<<<>>>><>>>><<<<>><<<><<<<><<>>><<<<><<<>>>><<<>>>><<>><<<<><>><><><><<>>><<<><<>><<>>>><<<>>><>><<<>>><<<>>><<>>><>>><<<<>>>><<<<>><<<<>>><<<>><<<>>>><<><>><><<><<>><<<>>>><<<><<>>>><<><<<<>>><<>><<<>>><<><<>>><<<>><<><<>><<><<><<<>>>><>>><<<><<<>>>><><<<>><<<>>><<><>>><<<<>>>><>><<<><>>><<>>>><<<><<<>><>>>><<<><<<<><>><<<<>>><<<<>><><><<<>>>><<<>>><<>>><<<<>>><<<<>>><<<<>>><<<<>>><<>>>><<>>>><<<<>>><<>><><>>>><<>>><>>>><<<>><<<<>>>><<<<>>><<<>>>><>><<<<>>>><<<<><<<<>>>><><>><>>><<>><>>>><<<<>>><<<><<<><<<>>>><<<>>>><><><<<>>>><>>><<<>><><<<<><<<<>><<<>><><<>>><<<<><<>>><<>>><<>>><>>>><<<>><><<<>>><<<<>>><<<<>><><<<<>><<<>>>><<<<>>><<<>>><<><<>>>><>><<<><<<>>>><<<><<<<>>><>>>><<>>><<<<>><<<<>>>><<<>>><<<>>><<<>>><<<>>><<<>>><><><<>><>>><<<>>><<><>>>><<>>><<<<>><<><<<<>>><<<<><<>><>><<<>><<<<><<<<>>><<><>><<<<>><<<>>>><<<>>>><<<<>><<<>>><<>>>><<<<><<<<>>>><<<<><<<<>>>><>>><<<<><<<><<<>>><<<>>>><>><<<>><<>>>><>>><><<>>><<<<>>>><>>><<<>><<>><<<>><<<<>><<<<><<<<><<<<><<<<>><<>><<<><<>>><<<<>>><>><<><>>>><<<>>><<<<>>><<>>><<<><<<<>><<>><<>>>><<<<><>>>><<<><<>>>><<<<>>>><<<>><<>>>><>><<<<>>><<<<><<>><<<<>>>><><><<>><<>>><<<>><<<<><<<>>>><<<>>><<<<><<<<><>>>><><<<>><<<>><>>><<><><<<><<><>>>><<<<>>><<<<><<<<>>><<<<>>><<<<><>>>><<<>>>><<<>>>><<<<><<>>><<>>><<<<>>>><<<<>>><<>>>><><><<><<>><<>><<><<>>>><>><><<<>>>><<<>><<<>>>><<>>>><>>><<>><<<<>><>>><<<><<<<><<<><<<<>>><<<>><<>>>><<<>>>><<<<>>>><<<<>>><>>><<<>>><>>>><<<<>><<<<>>>><<>>>><<>><<<<>>>><<>><>>><<<>>><>>>><<><<<<>><<>>><<<>>><<<><<<<><<<<>><<>><>><>><<>><<>>><<<<>><<<>><<<<>><>><<<>>>><>>>><>>><<>>><><<<<>>>><<>><<<<>>>><<>><<<>>><<<><<<<>>><<>><<<>>>><<<>>>><<>>>><>><<<>><>>>><<>>><<>>>><<<>>>><<>>><>>><<<>>>><<>><<<<>>><<>><<<>><>><<<><<<><<<>><<<<>><<<<>>>><<>><><>>><<<>>><<<<><<<>>>><<<<>>><>><<<>>><<<<>>><<<>>><><<><<>><>>><<>>><<<<>>>><<<<>>>><<<>>>><><<<>>>><<><<<<>>>><<>><>>><><<<>>><<<>>><<<>>>><<<<>>><>>><<<<><<<>>>><<<>>>><<<>>><<<<><<>>><>><<<<>><>><<<<><<>>>><<<>>>><<<<>>>><<>>><><<<>>><<<>><<<<>>>><<<<>>><<>>><<<>>>><>>>><<<>>><<>><<<<>><<<>><<<>><<<<>><<<>><<><<><>><><><<>>>><<>>><>>>><>><><>><><<>>><<>>>><>><<<<>><><<<><<<<>>><<>>>><<<<>>>><><>><<>>>><<><<<>>><>><<<<>>><<>><<><<>>>><<<<>>><<<>>><<<<><<<><<<><<<><>><<<>><<<><<<<><<<>><<><<<<>><<>><>>><<>>><>>>><><<>><<>><<>>>><>>><<<<>>>><<<>><>><<<>>><<<<>><<>>><<<<>>>><<<<>><<<><>><<><<<>>><<<<>>><<<>>><<>><<<>>><>>>><<<<>><<<<>><<>>>><<<>>>><<<><<<>><>>>><<><<<<>>>><<<><>>>><<>>><<<>>><<>>>><<<>>>><<<><<<>><<<><<>>><<<<>>><<><<<<>><<<><<<<>>>><<>>>><<<>><<>><<<>>>><<<><<<><<<<>>>><>>><<<>>><<<<>>><<<<><<<>>>><<<<>><<>><<><<<>>><<<>>><<<>><>>>><>><<><<<<>>>><>><><><<<<>><>><<<<>><><<<>>><<<<><>>><><>>><<<<>>><<>>><<<>>>><<<<>>><<<<><<<>>>><<>>>><>><<<>>><<<<>><<>>><<<<>>>><<<<><<>>>><<<>><<<>>><<<<><<>>><<>>><>>>><<<<>>><<<><<<>>>><>><<<>><<<>><><<<<><<>>>><<>><>>><<>><<<<>>>><<>><>>><<<<>><><<<<>><<<<>>><<<<>>>><<<><<>>><<<>><<<>>>><<>><<>><>>><<>>>><><<>><<>>>><<>>><>>><<<<>><<>>><<<>>><<><<<<>><<<><<<<>>>><<<>>><<<<>>><<<<>>><<<>>><<<<>>>><>>><<<<>>><<<>>>><<><<<<>>><<>>>><<<>>>><<<<>>>><<>>>><<><<<<>><><<<>>>><>>>><<>>>><><<<>>><<>>>><><<<>><<<>>><<<>>>><>>>><>>><<>>>><<<>>>><<>>><<<<><<<<>><>><<<>>>><<><<<>>><<<>><<>><<<<>>>><<><<<<><<>><<>>><>><<<>>><><<<>>>><<><<<>>>><<<><<<>><>>>><>>>><<<>>>><>>><>>><<<><<<<>>><<<<><<<><<>>>><<<>>>><<<<><<<><<<>>>><<<>>>><<>>>><<<><<>>>><><<>><<<><<<>>><<<>><<<<><<><<<<><<<>>>><<<>><<><<>><<><<<>><<>>>><<<>><<<<>><<<>>><>>>><<<<>>>><<>>>><>>>><<>><<>><<<><<<<>><<>>><<<<>>><<>>>><<><<<<><<<>>><<<<><<<>>><<>><<<<>>><<<<>>><<<<>>>><<>><<<<>>>><<<<>><<<<>>><<<<>>><<<>>>><<>><<<<>>><<><<<<>><<>>>><<<<><<>>>><>>>><>>><>>>><<<><<<<><<>><<<<>><<<>>>><<<<>>>><<>><<>>><<<><<<>><><<<<><<<<>>><<<<>>><<<>>><>><<><<>><<<>>>><<<<>><>><<><<<<>>><<<>>><<<<>><<>>>><>><>>>><<<>>><<<<>>><<><<>><<<>><<<><>>><><<>>><<<>>><<<>>><<<<>>>><<<>><<<>><<<>>><<<<>>>><<<><<>>><>>><>><>>>><<<>><<<><<<>>>><<<<><<<>><>>><>><<<><><<><<<>>><>><<<<>><<<<>>>><<>><>><<<<>><>><<<<>>><<<<>><>>><>>>><<<<>>>><<<><<<>>><>>>><<<<>>>><<<<>><<<><<<<>>>><<<>>><<><<<>>><<<>>>><>><<><<<><<<<>><<<>>><<<>>><<<>>><<<>><<<<>>>><<<<>><<>>><>>><<<><<>>><<<<><<<>>><<<<>>>><<<<><>><<>>>><<<>><>>><<><<<<>>><><<<><<>><<<<><<<>>>><><<><<>>><<>>><>>><>>><>>>><<><><><<>>>><>>>><<<<>><<<<>>>><><<<<>>><<<><<<<><><<>>><<<<>>><<<<>><<<<>>>><<><<<>>><<<<>>>><<<><<<<><>>><<<>>><>>>><<<>><<<<>>>><<<<>><<<><<<>>><><<>>><<>>><>>><<<<><<<>>><<<<><<<>>><<>>><<<><<>><><<>>><<<<>>><<<<>>>><<>><<<<><><<<><<<<>>>><<><>><<>>>><>>><<>>>><<><<<>>>><<<<>><<<><<>>>><<>>><<>><<<>>><<><<<<>>><<<<>>>><<<<><<<>><>>><<<>>>><<<><<<<><<>>><<>><>>>><>>><<<<>>>><<<<>><><>>>><<>>><<><<<>>><<<<><<<<>>><>>>><<<<><<<><<<<>>>><<>><<<<>>>><<<>><<>><>>><<<<>>>><>>><>>><<><>><<<><>><<<<>>>><><<>><<<>><>>><><<>>><<<<>><<><<<<><>>><<<><<<<>>".chars().cycle();
    // Configuring existing occupied coordinates
    let mut existing = HashSet::new();
    let mut top_layer = TopLayer {
        coords: Vec::new(),
        highest: datum,
    };
    type CurrentHeight = u64;
    type CurrentCount = u64;
    let mut top_layer_patterns: HashMap<TopLayerPattern, (CurrentCount, CurrentHeight)> =
        HashMap::new();
    let mut current_count = 0;

    while let Some(current_rock_shape) = rock_seq_iter.next() {
        // Creating the current rock
        let mut current_rock = Rock::new(current_rock_shape, datum);

        // Modelling rock interactions with wind and gravity
        loop {
            let jet_direction = match jet_stream_iter.next() {
                Some(c) => match c {
                    '>' => JetDirection::Right,
                    '<' => JetDirection::Left,
                    _ => unreachable!(),
                },
                None => unreachable!(),
            };
            match current_rock.pushed_by_jet(jet_direction, &existing) {
                Some(_) => {}
                None => {}
            };
            match current_rock.fall_downwards(&existing) {
                Some(_) => {}
                None => {
                    // Rock has come to a stop
                    current_count += 1;

                    // Checking the highest point and updating the datum
                    let rock_highest_point = current_rock.get_highest_point();
                    if rock_highest_point > datum {
                        datum = rock_highest_point
                    }

                    println!("{}", current_count);

                    match current_count {
                        c if c == LIMIT_COUNT => {
                            println!("Result: {}", datum);
                            return ();
                        }
                        _ => {}
                    }
                    // Updating the existing coordinates
                    existing.extend(current_rock.coords.iter()); // Adding the stopped coordinates into the existing coordinates

                    // Updating the top layer
                    current_rock.coords.iter().for_each(|c| {
                        top_layer.insert_coord(*c);
                    });

                    // Checking if this pattern has been encountered before
                    let current_pattern = top_layer.get_pattern_id();
                    match top_layer_patterns.get(&current_pattern) {
                        Some((prev_count, prev_height)) => {
                            // println!("prev count: {}, prev height: {}, current count: {}, current height: {}!", prev_count, prev_height, current_count, datum);
                            let height_delta = datum - prev_height;
                            let count_delta = current_count - prev_count;

                            let mut multiplier = 1000000;
                            match count_delta > 1000 {
                                true => {
                                    println!("{}, {}", count_delta, height_delta);
                                    loop {
                                        if multiplier == 1 {
                                            break;
                                        }
                                        if current_count + (count_delta * multiplier) >= LIMIT_COUNT
                                        {
                                            multiplier = multiplier / 10;
                                            println!("Reduced multiplier to {}", multiplier);
                                            continue;
                                        }
                                        println!(
                                            "Current count and height: {}, {}",
                                            current_count, datum
                                        );
                                        current_count += count_delta * multiplier;
                                        datum += height_delta * multiplier;
                                        existing = existing
                                            .into_iter()
                                            .map(|mut c| {
                                                c.y += height_delta * multiplier;
                                                c
                                            })
                                            .collect();
                                    }
                                }
                                false => {}
                            }

                            // while (current_count + (count_delta * 10000) < LIMIT_COUNT) & (count_delta > 1000) {
                            //     // println!("Incrementing the count and height using pattern: {}, {}", count_delta, height_delta);
                            //     println!("Current count and height: {}, {}", current_count, datum);
                            //     current_count += count_delta * 10000;
                            //     datum += height_delta * 10000;
                            //     existing = existing
                            //     .into_iter()
                            //     .map(|mut c| {
                            //         c.y += height_delta * 10000;
                            //         c
                            //     }).collect()
                            // }
                            top_layer_patterns.insert(current_pattern, (current_count, datum));
                        }
                        None => {
                            top_layer_patterns.insert(current_pattern, (current_count, datum));
                        }
                    };
                    break;
                }
            }
        }
    }

    // for (i, current_rock_shape) in rock_seq_iter.enumerate() {
    //     // Creating the current rock
    //     let mut current_rock = Rock::new(current_rock_shape, datum);

    //     // Modelling the rock getting blown by the wind & falling
    //     loop {
    //         let jet_direction = match jet_stream_iter.next() {
    //             Some(c) => match c {
    //                 '>' => JetDirection::Right,
    //                 '<' => JetDirection::Left,
    //                 _ => unreachable!(),
    //             },
    //             None => unreachable!(),
    //         };
    //         match current_rock.pushed_by_jet(jet_direction, &existing) {
    //             Some(_) => {}
    //             None => {}
    //         };
    //         match current_rock.fall_downwards(&existing) {
    //             Some(_) => {}
    //             None => {
    //                 // Rock has come to a stop
    //                 let rock_highest_point = current_rock.get_highest_point();
    //                 existing.extend(current_rock.coords.iter()); // Adding the stopped coordinates into the existing coordinates
    //                 if rock_highest_point > datum {
    //                     datum = rock_highest_point
    //                 }

    //                 // Adding the coordinates of the current rock into the top layer
    //                 // for c in current_rock.coords.iter() {
    //                 //     top_layer.insert_coord(*c);
    //                 // }
    //                 current_rock
    //                 .coords
    //                 .iter()
    //                 .for_each(|c| {
    //                     top_layer.insert_coord(*c);
    //                 });

    //                 // Getting the current pattern of the top layer
    //                 let top_layer_pattern = top_layer.get_pattern_id();

    //                 // Checking if pattern can be utilized
    //                 match top_layer_patterns.contains_key(&top_layer_pattern) {
    //                     true => {
    //                         let prev = *top_layer_patterns.get(&top_layer_pattern).unwrap();
    //                         let delta = datum - prev;
    //                         while
    //                     },
    //                     false => {
    //                         top_layer_patterns.insert(top_layer_pattern, datum);
    //                     },
    //                 }

    //                 break;
    //             }
    //         }
    //     }

    //     if (i + 1).rem_euclid(1000000) == 0 {
    //         println!("Number of rocks fallen: {}", i + 1)
    //     }

    //     if i + 1 == 1000000000000 {
    //         println!("Result: {}", datum);
    //         break;
    //     }
    // }
}
