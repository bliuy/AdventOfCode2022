// use serde::Deserialize;

use std::{cmp::Ordering, collections::BinaryHeap, fmt::Debug, io::BufRead};

#[derive(serde::Deserialize, Clone, Eq)]
#[serde(untagged)] // Allows Serde to decide on the correct variant for destructuring.
enum Value {
    Number(u16),
    List(Vec<Value>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => write!(f, "{arg0}"),
            Self::List(arg0) => f.debug_list().entries(arg0.into_iter()).finish(),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => unreachable!(),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => Some(lhs.cmp(rhs)),
            (Value::Number(lhs), rhs) => {
                let num_list = vec![Value::Number(*lhs)];
                Value::List(num_list).partial_cmp(&rhs)
            }
            (lhs, Value::Number(rhs)) => {
                let num_list = Value::List(vec![Value::Number(*rhs)]);
                lhs.partial_cmp(&num_list)
            }
            (Value::List(lhs), Value::List(rhs)) => {
                match lhs.len() {
                    0 => match rhs.len() {
                        0 => return Some(Ordering::Equal),
                        _ => return Some(Ordering::Less),
                    },
                    _ => match rhs.len() {
                        0 => {
                            // println!("RHS is 0 length. See LHS vs RHS: {:?} vs {:?}", lhs, rhs);
                            return Some(Ordering::Greater);
                        }
                        _ => {}
                    },
                }

                let left_max = lhs.len() - 1;
                let right_max = rhs.len() - 1;

                let mut i = 0;

                loop {
                    if i > left_max && i > right_max {
                        return Some(Ordering::Equal); // List of equal length - Proceed to check the next value.
                    } else if i > left_max {
                        return Some(Ordering::Less); // Left list runs out of values first.
                    } else if i > right_max {
                        println!(
                            // "RHS has run out of values. See LHS value vs RHS value: {:?} vs {:?}",
                            // lhs, rhs
                        );
                        return Some(Ordering::Greater); // Right list runs out of values first.
                    } else {
                        let left_val = lhs.get(i).unwrap();
                        let right_val = rhs.get(i).unwrap();
                        match left_val.partial_cmp(right_val).unwrap() {
                            Ordering::Less => return Some(Ordering::Less),
                            Ordering::Equal => {
                                i += 1;
                            }
                            Ordering::Greater => {
                                // println!("LHS value is > RHS value. See LHS value vs RHS value: {:?} vs {:?}", left_val, right_val);
                                return Some(Ordering::Greater);
                            }
                        };
                    }
                }
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::List(lhs), Self::List(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

fn binary_search<T>(arr: &[T], low: usize, high: usize, target: &T) -> Option<usize>
where
    T: Ord + Debug,
{
    // Value is not present within the array
    if low > high {
        return None;
    }

    let mid = (low + high) / 2;
    let mid_val = match arr.get(mid) {
        Some(val) => val,
        None => unreachable!(),
    };

    match target.cmp(mid_val) {
        Ordering::Less => {
            return binary_search(arr, low, mid - 1, target);
        }
        Ordering::Equal => return Some(mid),
        Ordering::Greater => {
            return binary_search(arr, mid + 1, high, target);
        }
    }
}

fn main() {
    // Reading the file
    // let file = std::fs::File::open("inputs/examples/example13.txt").expect("Failed to open file.");
    let file = std::fs::File::open("inputs/input13.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Reading the packets
    let mut heap = BinaryHeap::new();
    for rline in reader.lines() {
        let line = rline.unwrap();
        match line.len() {
            0 => {}
            _ => {
                let packet = serde_json::from_str::<Value>(&line).unwrap();
                heap.push(packet);
            }
        }
    }

    // Creating the divider packets
    let first_div = Value::List(vec![Value::List(vec![Value::Number(2)])]);
    let second_div = Value::List(vec![Value::List(vec![Value::Number(6)])]);
    heap.push(first_div.clone());
    heap.push(second_div.clone());

    // Getting the sorted vector
    let sorted = heap.into_sorted_vec();

    for (i, v) in sorted.clone().into_iter().enumerate() {
        println!("{} - {:?}", i, v)
    }

    // Finding the location of the first div
    let low = 0;
    let high = sorted.len() - 1;
    let first = match binary_search(&sorted, low, high, &first_div) {
        Some(val) => val + 1,
        None => panic!("Value should always be found."),
    };
    let second = match binary_search(&sorted, low, high, &second_div) {
        Some(val) => val + 1,
        None => panic!("Value should always be found."),
    };

    println!("Result: {}", first * second);
}
