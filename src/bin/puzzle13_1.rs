// use serde::Deserialize;

use std::{cmp::Ordering, fmt::Debug, io::BufRead};

#[derive(serde::Deserialize, Clone)]
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
                            println!("RHS is 0 length. See LHS vs RHS: {:?} vs {:?}", lhs, rhs);
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
                            "RHS has run out of values. See LHS value vs RHS value: {:?} vs {:?}",
                            lhs, rhs
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
                                println!("LHS value is > RHS value. See LHS value vs RHS value: {:?} vs {:?}", left_val, right_val);
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

#[derive(Debug, Clone)]
struct PacketPair(Option<Value>, Option<Value>);

fn main() {
    // Reading the file
    // let file = std::fs::File::open("inputs/examples/example13.txt").expect("Failed to open file.");
    let file = std::fs::File::open("inputs/input13.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    let mut i = 0;
    let mut packet_pair = PacketPair(None, None);
    let mut packet_pairs = Vec::new();

    for rline in reader.lines() {
        let line = rline.unwrap();

        // Completed packet
        if line.len() == 0 {
            packet_pairs.push(packet_pair.clone());
            i = 0;
            packet_pair.0 = None;
            packet_pair.1 = None;
            continue;
        }

        let val: Value = serde_json::from_str(&line).unwrap();
        if i == 0 {
            packet_pair.0 = Some(val)
        } else if i == 1 {
            packet_pair.1 = Some(val)
        } else {
            unreachable!()
        }

        i += 1;
    }

    // Performing the comparison
    let mut total: usize = 0;
    for (idx, pair) in packet_pairs.into_iter().enumerate() {
        let left_pkt = match pair.0 {
            Some(pkt) => {
                match pkt {
                    Value::Number(_) => panic!(), // Expecting a List for the packet layer.
                    Value::List(lhs) => lhs,
                }
            }
            None => panic!(),
        };
        let right_pkt = match pair.1 {
            Some(pkt) => {
                match pkt {
                    Value::Number(_) => panic!(), // Expecting a List for the packet layer.
                    Value::List(rhs) => rhs,
                }
            }
            None => panic!(),
        };

        match left_pkt.len() {
            0 => match right_pkt.len() {
                0 => panic!(),
                _ => {
                    total += idx + 1;
                    continue;
                }
            },
            _ => match right_pkt.len() {
                0 => {
                    continue;
                }
                _ => {}
            },
        }

        let left_max = left_pkt.len() - 1;
        let right_max = right_pkt.len() - 1;

        let mut i = 0;

        loop {
            if (i > left_max) & (i > right_max) {
                // Indicates that the payloads are identical - Should never happen.
                panic!()
            } else if i > left_max {
                // Indicates that the left payload has run out of values - right order.
                total += idx + 1;
                break;
            } else if i > right_max {
                // Indicates that the right payload has run out of values - wrong order.
                break;
            } else {
                let left_val = left_pkt.get(i).unwrap();
                let right_val = right_pkt.get(i).unwrap();
                match left_val.partial_cmp(right_val).unwrap() {
                    Ordering::Less => {
                        total += idx + 1;
                        break;
                    }
                    Ordering::Equal => {
                        i += 1;
                    }
                    Ordering::Greater => {
                        println!("Packet index: {}", idx);
                        // println!("Left pkt is greater than right pkt. See left pkt vs right pkt: {:?} vs {:?}", left_pkt, right_pkt);
                        break;
                    }
                }
            }
        }
    }

    println!("Result: {}", total);
}
