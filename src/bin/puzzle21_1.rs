use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Add, Mul, Sub},
};

type MonkeyName = String;
type NameLookup = HashMap<MonkeyName, JobType>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum JobType {
    Add(MonkeyName, MonkeyName),
    Minus(MonkeyName, MonkeyName),
    Multiply(MonkeyName, MonkeyName),
    Divide(MonkeyName, MonkeyName),
    Value(i64),
}

fn evaluate(name: &MonkeyName, lookup: &mut NameLookup) {
    let job = lookup.get(name).unwrap().clone();
    match job {
        JobType::Add(first, second) => {
            let a: i64;
            let b: i64;
            match lookup.get(&first).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&first, lookup),
            }
            match lookup.get(&second).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&second, lookup),
            }
            match lookup.get(&first).unwrap() {
                JobType::Value(val) => {
                    a = *val;
                }
                _ => unreachable!(),
            }
            match lookup.get(&second).unwrap() {
                JobType::Value(val) => {
                    b = *val;
                }
                _ => unreachable!(),
            }
            *lookup.get_mut(name).unwrap() = JobType::Value(a.add(b));
        }
        JobType::Minus(first, second) => {
            let a: i64;
            let b: i64;
            match lookup.get(&first).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&first, lookup),
            }
            match lookup.get(&second).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&second, lookup),
            }
            match lookup.get(&first).unwrap() {
                JobType::Value(val) => {
                    a = *val;
                }
                _ => unreachable!(),
            }
            match lookup.get(&second).unwrap() {
                JobType::Value(val) => {
                    b = *val;
                }
                _ => unreachable!(),
            }
            *lookup.get_mut(name).unwrap() = JobType::Value(a.sub(b));
        }
        JobType::Multiply(first, second) => {
            let a: i64;
            let b: i64;
            match lookup.get(&first).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&first, lookup),
            }
            match lookup.get(&second).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&second, lookup),
            }
            match lookup.get(&first).unwrap() {
                JobType::Value(val) => {
                    a = *val;
                }
                _ => unreachable!(),
            }
            match lookup.get(&second).unwrap() {
                JobType::Value(val) => {
                    b = *val;
                }
                _ => unreachable!(),
            }
            *lookup.get_mut(name).unwrap() = JobType::Value(a.mul(b));
        }
        JobType::Divide(first, second) => {
            let a: i64;
            let b: i64;
            match lookup.get(&first).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&first, lookup),
            }
            match lookup.get(&second).unwrap().clone() {
                JobType::Value(_) => {}
                _ => evaluate(&second, lookup),
            }
            match lookup.get(&first).unwrap() {
                JobType::Value(val) => {
                    a = *val;
                }
                _ => unreachable!(),
            }
            match lookup.get(&second).unwrap() {
                JobType::Value(val) => {
                    b = *val;
                }
                _ => unreachable!(),
            }
            *lookup.get_mut(name).unwrap() = JobType::Value(a.div_euclid(b));
        }
        JobType::Value(_) => {}
    }
}

const OPERATION_PATTERN: &str = r"([a-z]+): ([a-z]+) ([+|\-|*|/]{1}) ([a-z]+)";
const VALUE_PATTERN: &str = r"([a-z]+): ([0-9]+)";

fn main() {
    // Reading inputs
    // let file = std::fs::File::open("inputs/examples/example21.txt").expect("Failed to open file.");
    let file = std::fs::File::open("inputs/input21.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Compiling the regex pattern
    let operation_pattern = regex::Regex::new(OPERATION_PATTERN).unwrap();
    let value_pattern = regex::Regex::new(VALUE_PATTERN).unwrap();

    let mut lookup = HashMap::new();
    for rline in reader.lines() {
        let line = rline.unwrap();

        // Seeing which regex patterns induces a match
        let (name, val) = match value_pattern.captures(&line) {
            Some(matched) => {
                let result: i64 = matched.get(2).unwrap().as_str().parse().unwrap();
                let name = matched.get(1).unwrap().as_str().to_owned();
                (name, JobType::Value(result))
            }
            None => match operation_pattern.captures(&line) {
                Some(matched) => {
                    let name = matched.get(1).unwrap().as_str().to_owned();
                    let first = matched.get(2).unwrap().as_str().to_owned();
                    let second = matched.get(4).unwrap().as_str().to_owned();
                    let operator = matched.get(3).unwrap().as_str();
                    let result = match operator {
                        "+" => JobType::Add(first, second),
                        "-" => JobType::Minus(first, second),
                        "*" => JobType::Multiply(first, second),
                        "/" => JobType::Divide(first, second),
                        _ => unreachable!(),
                    };
                    (name, result)
                }
                None => unreachable!(),
            },
        };
        lookup.insert(name, val);
    }

    // Evaluating the root
    let starting_name = "root".to_owned();
    evaluate(&starting_name, &mut lookup);

    println!("{:?}", lookup.get(&starting_name).unwrap());
}
