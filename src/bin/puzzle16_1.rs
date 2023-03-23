use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::BufRead,
    result,
};

use regex::{Match, Regex};

const REGEX_PATTERN: &str = r"Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? ((?:[A-Z]{2})(?:,\s*[A-Z]{2})*)";

fn backtrack(
    valve: String,
    current_time: u32,
    accumulated_pressure: u32,
    opened: &mut HashSet<String>,
    flowrates: &HashMap<String, u32>,
    neighbours: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> u32 {
    let valve_neighbours = neighbours.get(&valve).unwrap();

    match current_time {
        0 | 1 => {
            return accumulated_pressure; // No further actions, opening valves would not be useful as well.
        }
        _ => {
            // Enough time to open the current valve
            let updated_total_pressure: Option<u32>;

            // Seeing if the current valve has already been opened
            match opened.contains(&valve) {
                true => updated_total_pressure = None,
                false => {
                    let additional_pressure = *flowrates.get(&valve).unwrap();
                    updated_total_pressure =
                        Some(accumulated_pressure + (additional_pressure * (current_time - 1)));
                    // Only effective after 1 min of opening the valve.
                }
            }

            // If all nodes have been visited, no point of further exploration
            match visited.len() == neighbours.keys().len() {
                true => match updated_total_pressure {
                    Some(updated_pressure) => return updated_pressure,
                    None => return accumulated_pressure,
                },
                false => {}
            }

            // Exploring all other neighbours
            let mut results = Vec::new();
            for nei in valve_neighbours {
                visited.insert(nei.clone());
                let closed_result = backtrack(
                    nei.clone(),
                    current_time - 1, // No valve opened, 1 min to move to next valve.
                    accumulated_pressure, // No increase in pressure
                    opened,
                    flowrates,
                    neighbours,
                    visited,
                );
                let opened_result = match updated_total_pressure {
                    Some(updated_pressure) => {
                        opened.insert(valve.clone());
                        backtrack(
                            nei.clone(),
                            current_time - 2, // 1 min to open current valve, 1 min to move to next valve.
                            updated_pressure,
                            opened,
                            flowrates,
                            neighbours,
                            visited,
                        )
                    }
                    None => {
                        0 // Only consider the closed result
                    }
                };
                results.push(closed_result);
                results.push(opened_result);
                visited.remove(nei);
            }
            results.into_iter().max().unwrap()
        }
    }
}
// fn backtrack(
//     valve: String,
//     remaining_time: u32,
//     visited: &mut HashSet<String>,
//     total_pressure: u32,
//     flowrates: &HashMap<String, u32>,
//     neighbours: &HashMap<String, Vec<String>>,
// ) -> u32 {
//     // Defining the termination condition

//     if remaining_time <= 1 {
//         return total_pressure;
//     } else {
//         let current_time = remaining_time - 1;
//     }

//     let current_time = match remaining_time <= 1 {
//         true => return total_pressure,
//         false => remaining_time - 1,
//     }

//     // Processing the current valve
//     let valve_flowrate = *flowrates.get(&valve).unwrap();
//     let valve_neighbours = neighbours.get(&valve).unwrap();
//     let valve_pressure = match current_time > 0 {
//         true => (remaining_time - 1) * valve_flowrate, // Enough time to open the current valve.
//         false => 0, // Insufficient time to open the current valve.
//     }
//     let valve_pressure = (remaining_time - 1) * valve_flowrate;

//     // Identifying potential neighbours
//     let potential_neighbours = valve_neighbours
//         .into_iter()
//         .filter(|&x| !visited.contains(x))
//         .collect::<Vec<_>>();

//     // No neighbours left to visit - Simply waiting it out
//     // println!(
//     //     "{}, current pressure = {} -> {:?}",
//     //     valve, updated_total_pressure, potential_neighbours
//     // );
//     if potential_neighbours.len() == 0 {
//         return total_pressure + valve_pressure;
//     }

//     // Performing backtracking
//     // let mut max_result = 0; // Finding the maximum value
//     let mut results = Vec::new();
//     for neighbour in potential_neighbours {
//         // Adding the neighbour that is about to be visited
//         visited.insert(neighbour.clone());

//         // Assuming that the current valve is opened
//         let result_of_opened_valve = backtrack(
//             neighbour.clone(),
//             current_time - 1, //  1 min opening valve + 1 min to reach subsequent valve
//             visited,
//             total_pressure + valve_pressure, // current valve pressure was released.
//             flowrates,
//             neighbours,
//         );
//         results.push(result_of_opened_valve);

//         // Assuming that the current valve is not opened - Simply moving to next valve without opening the valve
//         let result_of_closed_valve = backtrack(
//             neighbour.clone(),
//             remaining_time - 1, // 1 min to reach the next valve
//             visited,
//             total_pressure, // No change to the pressure
//             flowrates,
//             neighbours,
//         );
//         results.push(result_of_closed_valve);

//         // Removing the neighbour from the visited.
//         visited.remove(neighbour);
//     }

//     // Returning the maximum result
//     results.into_iter().max().unwrap()
// }

fn main() {
    let file = std::fs::File::open("inputs/examples/example16.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);
    let regex_pattern = match Regex::new(REGEX_PATTERN) {
        Ok(x) => x,
        Err(_) => unreachable!(),
    };

    // Graph creation
    let mut flowrates = HashMap::new();
    let mut neighbours = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    for rline in reader.lines() {
        let line = match rline {
            Ok(x) => x,
            Err(_) => unreachable!(),
        };
        let matched = match regex_pattern.captures(&line) {
            Some(x) => x,
            None => unreachable!(),
        };
        let valve = match matched.get(1) {
            Some(x) => x.as_str().to_owned(),
            None => unreachable!(),
        };
        let flowrate: u32 = match matched.get(2) {
            Some(x) => match x.as_str().parse() {
                Ok(y) => y,
                Err(_) => unreachable!(),
            },
            None => unreachable!(),
        };
        let neighbours_list = match matched.get(3) {
            Some(x) => x.as_str(),
            None => unreachable!(),
        };

        // Updating the flowrate table
        flowrates.insert(valve.clone(), flowrate);

        // Updating the neighbours table
        let neighbour = neighbours_list
            .split(",")
            .map(|x| x.trim().to_owned())
            .collect::<Vec<_>>();
        neighbours.insert(valve, neighbour);
    }

    let mut opened = HashSet::new();
    let mut visited = HashSet::new();
    let result = backtrack(
        "AA".to_owned(),
        30,
        0,
        &mut opened,
        &flowrates,
        &neighbours,
        &mut visited,
    );

    println!("Result: {}", result);
}
