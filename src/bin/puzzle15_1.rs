use regex;
use std::{
    backtrace::BacktraceStatus,
    collections::{HashSet, VecDeque},
    hash::Hash,
    io::BufRead,
    str::FromStr,
};

type ManhattanDistance = u32;

type ExclusionZone = HashSet<Coords>;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Coords {
    x: i32,
    y: i32,
}

// Not required - Coords can be derived to be Copy - Done purely for lifetimes experimentation
impl<'a, 'b> std::ops::Sub<&'a Coords> for &'b Coords {
    type Output = ManhattanDistance;

    fn sub(self, rhs: &'a Coords) -> Self::Output {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = rhs.x;
        let y2 = rhs.y;
        let x = x1.abs_diff(x2);
        let y = y1.abs_diff(y2);
        x + y
    }
}

impl std::ops::Sub for Coords {
    type Output = ManhattanDistance;

    fn sub(self, rhs: Coords) -> Self::Output {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = rhs.x;
        let y2 = rhs.y;
        let x = x1.abs_diff(x2);
        let y = y1.abs_diff(y2);
        x + y
    }
}

struct Beacon {
    coords: Coords,
}

struct Sensor {
    coords: Coords,
    nearest: Beacon,
    distance_to_beacon: ManhattanDistance,
}

impl Sensor {
    fn check_surroundings(
        starting: Coords,
        zones: &mut ExclusionZone,
        max_distance: ManhattanDistance,
    ) {
        // Adding the current zone into the list of visited zones
        let mut visiting = VecDeque::new(); // Contains zones that needs to be visited
        visiting.push_back(starting);
        let mut distance_to_target = starting.y.abs_diff(2000000);

        while let Some(current) = visiting.pop_front() {
            // Adding the zone into list of visited zones
            zones.insert(current);

            // Checking if zone is closer to target
            if current.y.abs_diff(2000000) > distance_to_target {
                continue;
            } else {
                distance_to_target = current.y.abs_diff(2000000)
            }

            // Steps to next zones
            let positional_modifiers = [
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ];

            // Identifying the potential zones
            let potential_zones = positional_modifiers
                .into_iter()
                .map(|positional_modifier| {
                    // Constructing the next zone to visit
                    let mut zone = current;
                    zone.x += positional_modifier.0;
                    zone.y += positional_modifier.1;
                    zone
                })
                .filter(|zone| {
                    !zones.contains(zone) // Only visiting if it hasn't been visited before.
                })
                .filter(|zone| {
                    zone - &starting < max_distance // Only visits the zone if it falls within the maximum allowable distance.
                })
                .collect::<Vec<_>>();

            // Adding the potential zones into the list of zones to be visited
            visiting.extend(potential_zones.into_iter());
        }
    }

    fn new(coords: Coords, closest_beacon: Beacon) -> Self {
        let distance_to_beacon = &coords - &closest_beacon.coords;
        Sensor {
            coords: coords,
            nearest: closest_beacon,
            distance_to_beacon: distance_to_beacon,
        }
    }

    fn get_exclusion_zone(&self) -> ExclusionZone {
        let starting_coord = self.coords;
        let mut exclusion_zones = HashSet::new();
        Sensor::check_surroundings(
            starting_coord,
            &mut exclusion_zones,
            self.distance_to_beacon,
        );
        exclusion_zones
    }
}

fn main() {
    // Opening and loading the file into the ReadBuffer
    let opened_file = std::fs::File::open("inputs/alt/input15.txt").expect("Missing input file!");
    // let opened_file = std::fs::File::open("inputs/examples/example15.txt").expect("Missing input file!");
    let reader = std::io::BufReader::new(opened_file);

    // Compiling the regex pattern
    let pattern = regex::Regex::new(
        "Sensor at x=(-?[0-9]*), y=(-?[0-9]*): closest beacon is at x=(-?[0-9]*), y=(-?[0-9]*)",
    )
    .unwrap();

    let mut exclusion_zones = HashSet::new();
    let mut sensors = Vec::new();

    for rline in reader.lines() {
        let line = rline.unwrap();
        println!("{}", line);
        let matched = pattern.captures(&line).unwrap();
        let sensor_x: i32 = matched.get(1).unwrap().as_str().parse().unwrap();
        let sensor_y: i32 = matched.get(2).unwrap().as_str().parse().unwrap();
        let beacon_x: i32 = matched.get(3).unwrap().as_str().parse().unwrap();
        let beacon_y: i32 = matched.get(4).unwrap().as_str().parse().unwrap();

        // Constructing the beacon
        let beacon = Beacon {
            coords: Coords {
                x: beacon_x,
                y: beacon_y,
            },
        };
        // Constructing the sensor
        let sensor = Sensor::new(
            Coords {
                x: sensor_x,
                y: sensor_y,
            },
            beacon,
        );

        // Finding the exclusion zones for this sensor
        let exclusion_zone = sensor.get_exclusion_zone();
        exclusion_zones.extend(exclusion_zone.into_iter());

        sensors.push(sensor);
    }

    // println!("{:#?}", exclusion_zones);

    let beacon_coords = sensors
        .iter()
        .map(|sensor| sensor.nearest.coords)
        .collect::<HashSet<_>>();

    let counted = exclusion_zones
        .difference(&beacon_coords)
        .into_iter()
        .filter(|&&coord| coord.y == 2000000)
        .collect::<Vec<_>>()
        .len();

    println!("Result: {}", counted);
}
