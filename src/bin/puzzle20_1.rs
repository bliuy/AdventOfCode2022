use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    ops::Mul,
    slice::SliceIndex,
};

// type Array = Vec<i32>;
// type InitialPosition = usize;
// type CurrentPosition = usize;
// type Positions = HashMap<InitialPosition, CurrentPosition>;
// type CurrentPositions = HashMap<i32, usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UniqueValue {
    id: usize,
    val: ToMove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ToMove {
    Forward(usize),
    Back(usize),
}

impl ToMove {
    fn new(v: i32) -> Self {
        match v.cmp(&0) {
            std::cmp::Ordering::Less => ToMove::Back(v.abs() as usize),
            std::cmp::Ordering::Equal => ToMove::Forward(0),
            std::cmp::Ordering::Greater => ToMove::Forward(v as usize),
        }
    }
}

// fn move_number(mut array: &mut Array, current_positions: &mut CurrentPositions, moved: Moved) {
//     let current_position =
//     match moved {
//         Moved::Forward(mut steps) => {
//             while steps > 0 {

//                 steps = steps - 1;
//             }
//         },
//         Moved::Back(_) => todo!(),
//     }
// }

fn main() {
    // Reading inputs
    let file = std::fs::File::open("inputs/examples/example20.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);
    let mut array = Vec::new();
    let mut positions = HashMap::new(); // Used to keep track of the current position of the unique value.

    // Populating the array
    for (i, rline) in reader.lines().enumerate() {
        let line = rline.unwrap();
        let value: i32 = line.parse().unwrap();

        // Constructing the unique values
        let to_move = ToMove::new(value);
        let unique_value: UniqueValue = UniqueValue {
            id: i,
            val: to_move,
        };
        positions.insert(unique_value, i);
        array.push(unique_value);
    }

    // // Capturing the state of the initial array
    let starting_array = array.clone();

    for uv in starting_array {
        match uv.val {
            ToMove::Forward(mut steps) => {
                // Getting the starting position
                let mut current_position = *positions.get(&uv).unwrap();

                while steps > 0 {
                    // Getting the future position
                    let future_position = match current_position == array.len() - 1 {
                        true => 0,
                        false => current_position + 1,
                    };
                    let future_position_value = *array.get(future_position).unwrap();

                    // Performing the swaps
                    array.swap(current_position, future_position);

                    // Updating the position records
                    *positions.get_mut(&future_position_value).unwrap() = current_position;
                    *positions.get_mut(&uv).unwrap() = future_position;

                    // Updating the current position
                    current_position = future_position;

                    steps = steps - 1;
                }
            }
            ToMove::Back(mut steps) => {
                // Getting the starting position
                let mut current_position = *positions.get(&uv).unwrap();

                while steps > 0 {
                    // Getting the future position
                    let future_position = match current_position == 0 {
                        true => array.len() - 1,
                        false => current_position - 1,
                    };
                    let future_position_value = *array.get(future_position).unwrap();

                    // Performing the swaps
                    array.swap(current_position, future_position);

                    // Updating the position records
                    *positions.get_mut(&future_position_value).unwrap() = current_position;
                    *positions.get_mut(&uv).unwrap() = future_position;

                    // Updating the current position
                    current_position = future_position;

                    steps = steps - 1;
                }
            }
        }

        let temp_array = array.clone().into_iter().map(|x| x.val).collect::<Vec<_>>();
        println!("{:?}", temp_array);
    }
}
