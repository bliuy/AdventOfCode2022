use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

fn main() {
    // Hardcoding the shape of the dataset

    // Reading the file
    let file = std::fs::File::open("inputs/input12.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Constructing the map
    let mut map = Vec::new();

    for rline in reader.lines() {
        let line = rline.unwrap();
        let row = line.chars().collect::<Vec<_>>();
        map.push(row);
    }

    let max_i = map.len() - 1; // Maximum index on the rows
    let max_j = map[0].len() - 1; // Maximum index on the columns

    // Creating the graph model
    let mut graph = HashMap::new();
    let mut elevations = HashMap::new();
    let mut steps = HashMap::new();
    let mut starting = Position { x: 0, y: 0 };
    let mut previous = HashMap::new();

    // Populating the graph
    for i in 0..=max_i {
        for j in 0..=max_j {
            let val = match *map.get(i).unwrap().get(j).unwrap() {
                'S' => {
                    starting = Position { x: i, y: j };
                    'a'
                }
                'E' => {
                    '{' // Purely for ensuring a higher value during cmp ops.
                }
                c => c,
            };
            let current = Position { x: i, y: j };
            let mut neighbours = Vec::new();
            if i > 0 {
                neighbours.push(
                    Position { x: i - 1, y: j }, // Moving up
                )
            }
            if i < max_i {
                neighbours.push(
                    Position { x: i + 1, y: j }, // Moving down
                )
            }
            if j > 0 {
                neighbours.push(
                    Position { x: i, y: j - 1 }, // Moving left
                )
            }
            if j < max_j {
                neighbours.push(
                    Position { x: i, y: j + 1 }, // Moving right
                )
            }

            graph.insert(current, neighbours);
            elevations.insert(current, val);
            steps.insert(current, 0);
            previous.insert(current, None);
        }
    }

    // Setting up BFS
    let mut queue = VecDeque::new();
    queue.push_back(starting);
    let mut visited = HashSet::new();
    let mut i = 0;

    // BFS
    let mut end_position = Position { x: 0, y: 0 };
    loop {
        i += 1;
        // Getting the current node to be processed
        let current = match queue.pop_front() {
            Some(node) => {
                // Checking if the node has been visited before
                if visited.contains(&node) {
                    continue; // No point revisiting this node
                }

                // Checking if we have reached the destination node
                if elevations.get(&node).unwrap() == &'{' {
                    end_position = node;
                    break;
                }

                // If not, return the node for further processing
                node
            }
            None => {
                println!("Terminated after {} loops", i);
                break;
            }
        };

        // Adding the current node to the visited hashset
        visited.insert(current);

        // Processing each of the neighbours
        let step = *steps.get(&current).unwrap();
        for neighbour in graph.get(&current).unwrap() {
            // Check - Can we move to the neighbour at all?
            if *elevations.get(neighbour).unwrap() as u32
                > *elevations.get(&current).unwrap() as u32 + 1
            {
                continue;
            }

            // Check - Has the neighbour been visited before?
            if visited.contains(&neighbour) {
                continue;
            }

            // Append the neighbour to the end of the deque
            queue.push_back(*neighbour); // Impl Copy - Allows for borrow deref.
            steps.insert(*neighbour, step + 1); // 1 Step away from the current node
            previous.insert(*neighbour, Some(current));
        }
    }

    // Creating the path
    let mut current = end_position;
    println!("{:?} - {}", current, elevations.get(&current).unwrap());
    loop {
        let prev = match previous.get(&current).unwrap() {
            Some(x) => *x,
            None => break,
        };
        println!(
            "{:?} - {}, steps taken: {}",
            prev,
            elevations.get(&prev).unwrap(),
            steps.get(&prev).unwrap()
        );
        current = prev;
    }
    println!("Result: {}", steps.get(&end_position).unwrap());
}
