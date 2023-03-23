use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Plus(Box<Node>, Box<Node>),
    Minus(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Value(i64),
    Variable(char),
    ValueAndVariable(i64, char),
}

impl Add for Node {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Node::Plus(first, second) => (*first + *second) + rhs,
            Node::Minus(first, second) => (*first - *second) + rhs,
            Node::Multiply(first, second) => (*first * *second) + rhs,
            Node::Divide(first, second) => (*first / *second) + rhs,
            Node::Value(lhs) => match rhs {
                Node::Plus(first, second) => (*first + *second) + Node::Value(lhs),
                Node::Minus(first, second) => (*first - *second) + Node::Value(lhs),
                Node::Multiply(first, second) => (*first * *second) + Node::Value(lhs),
                Node::Divide(first, second) => (*first / *second) + Node::Value(lhs),
                Node::Value(rhs) => Node::Value(lhs + rhs),
                Node::Variable(rhs) => Node::ValueAndVariable(lhs, rhs),
                Node::ValueAndVariable(rhs_val, rhs_var) => {
                    Node::ValueAndVariable(lhs + rhs_val, rhs_var)
                }
            },
            Node::Variable(lhs) => match rhs {
                Node::Plus(first, second) => (*first + *second) + Node::Variable(lhs),
                Node::Minus(first, second) => (*first - *second) + Node::Variable(lhs),
                Node::Multiply(first, second) => (*first * *second) + Node::Variable(lhs),
                Node::Divide(first, second) => (*first / *second) + Node::Variable(lhs),
                Node::Value(rhs) => Node::ValueAndVariable(rhs, lhs),
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
            Node::ValueAndVariable(first_lhs, first_rhs) => match rhs {
                Node::Plus(first, second) => {
                    (*first + *second) + Node::ValueAndVariable(first_lhs, first_rhs)
                }
                Node::Minus(first, second) => {
                    (*first - *second) + Node::ValueAndVariable(first_lhs, first_rhs)
                }
                Node::Multiply(first, second) => {
                    (*first * *second) + Node::ValueAndVariable(first_lhs, first_rhs)
                }
                Node::Divide(first, second) => {
                    (*first / *second) + Node::ValueAndVariable(first_lhs, first_rhs)
                }
                Node::Value(second_rhs) => {
                    Node::ValueAndVariable(first_lhs + second_rhs, first_rhs)
                }
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

impl Sub for Node {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Node::Plus(first, second) => (*first + *second) - rhs,
            Node::Minus(first, second) => (*first - *second) - rhs,
            Node::Multiply(first, second) => (*first * *second) - rhs,
            Node::Divide(first, second) => (*first / *second) - rhs,
            Node::Value(lhs) => match rhs {
                Node::Plus(first, second) => Node::Value(lhs) - (*first + *second),
                Node::Minus(first, second) => Node::Value(lhs) - (*first - *second),
                Node::Multiply(first, second) => Node::Value(lhs) - (*first * *second),
                Node::Divide(first, second) => Node::Value(lhs) - (*first / *second),
                Node::Value(rhs) => Node::Value(lhs - rhs),
                Node::Variable(rhs) => Node::ValueAndVariable(lhs, rhs),
                Node::ValueAndVariable(rhs_val, rhs_var) => {
                    Node::ValueAndVariable(lhs - rhs_val, rhs_var)
                }
            },
            Node::Variable(lhs) => match rhs {
                Node::Plus(first, second) => Node::Variable(lhs) - (*first + *second),
                Node::Minus(first, second) => Node::Variable(lhs) - (*first - *second),
                Node::Multiply(first, second) => Node::Variable(lhs) - (*first * *second),
                Node::Divide(first, second) => Node::Variable(lhs) - (*first / *second),
                Node::Value(rhs) => Node::ValueAndVariable(rhs, lhs),
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
            Node::ValueAndVariable(first_lhs, first_rhs) => match rhs {
                Node::Plus(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) - (*first + *second)
                }
                Node::Minus(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) - (*first - *second)
                }
                Node::Multiply(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) - (*first * *second)
                }
                Node::Divide(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) - (*first / *second)
                }
                Node::Value(second_rhs) => {
                    Node::ValueAndVariable(first_lhs - second_rhs, first_rhs)
                }
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

impl Mul for Node {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Node::Plus(first, second) => (*first + *second) * rhs,
            Node::Minus(first, second) => (*first - *second) * rhs,
            Node::Multiply(first, second) => (*first * *second) * rhs,
            Node::Divide(first, second) => (*first / *second) * rhs,
            Node::Value(lhs) => match rhs {
                Node::Plus(first, second) => Node::Value(lhs) * (*first + *second),
                Node::Minus(first, second) => Node::Value(lhs) * (*first - *second),
                Node::Multiply(first, second) => Node::Value(lhs) * (*first * *second),
                Node::Divide(first, second) => Node::Value(lhs) * (*first / *second),
                Node::Value(rhs) => Node::Value(lhs * rhs),
                Node::Variable(rhs) => Node::ValueAndVariable(lhs, rhs),
                Node::ValueAndVariable(rhs_val, rhs_var) => {
                    Node::ValueAndVariable(lhs * rhs_val, rhs_var)
                }
            },
            Node::Variable(lhs) => match rhs {
                Node::Plus(first, second) => Node::Variable(lhs) * (*first + *second),
                Node::Minus(first, second) => Node::Variable(lhs) * (*first - *second),
                Node::Multiply(first, second) => Node::Variable(lhs) * (*first * *second),
                Node::Divide(first, second) => Node::Variable(lhs) * (*first / *second),
                Node::Value(rhs) => Node::ValueAndVariable(rhs, lhs),
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
            Node::ValueAndVariable(first_lhs, first_rhs) => match rhs {
                Node::Plus(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) * (*first + *second)
                }
                Node::Minus(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) * (*first - *second)
                }
                Node::Multiply(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) * (*first * *second)
                }
                Node::Divide(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) * (*first / *second)
                }
                Node::Value(second_rhs) => {
                    Node::ValueAndVariable(first_lhs * second_rhs, first_rhs)
                }
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

impl Div for Node {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Node::Plus(first, second) => (*first + *second) / rhs,
            Node::Minus(first, second) => (*first - *second) / rhs,
            Node::Multiply(first, second) => (*first * *second) / rhs,
            Node::Divide(first, second) => (*first / *second) / rhs,
            Node::Value(lhs) => match rhs {
                Node::Plus(first, second) => Node::Value(lhs) / (*first + *second),
                Node::Minus(first, second) => Node::Value(lhs) / (*first - *second),
                Node::Multiply(first, second) => Node::Value(lhs) / (*first * *second),
                Node::Divide(first, second) => Node::Value(lhs) / (*first / *second),
                Node::Value(rhs) => Node::Value(lhs / rhs),
                Node::Variable(rhs) => Node::ValueAndVariable(lhs, rhs),
                Node::ValueAndVariable(rhs_val, rhs_var) => {
                    Node::ValueAndVariable(lhs / rhs_val, rhs_var)
                }
            },
            Node::Variable(lhs) => match rhs {
                Node::Plus(first, second) => Node::Variable(lhs) / (*first + *second),
                Node::Minus(first, second) => Node::Variable(lhs) / (*first - *second),
                Node::Multiply(first, second) => Node::Variable(lhs) / (*first * *second),
                Node::Divide(first, second) => Node::Variable(lhs) / (*first / *second),
                Node::Value(rhs) => Node::ValueAndVariable(rhs, lhs),
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
            Node::ValueAndVariable(first_lhs, first_rhs) => match rhs {
                Node::Plus(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) / (*first + *second)
                }
                Node::Minus(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) / (*first - *second)
                }
                Node::Multiply(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) / (*first * *second)
                }
                Node::Divide(first, second) => {
                    Node::ValueAndVariable(first_lhs, first_rhs) / (*first / *second)
                }
                Node::Value(second_rhs) => {
                    Node::ValueAndVariable(first_lhs / second_rhs, first_rhs)
                }
                Node::Variable(_) => unreachable!(),
                Node::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

const OPERATION_PATTERN: &str = r"([a-z]+): ([a-z]+) ([+|\-|*|/]{1}) ([a-z]+)";
const VALUE_PATTERN: &str = r"([a-z]+): ([0-9]+)";

fn main() {
    // Reading inputs
    // let file = std::fs::File::open("inputs/examples/example21.txt").expect("Failed to open file.");
    let file = std::fs::File::open("inputs/alt/input21.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Compiling the regex pattern
    let operation_pattern = regex::Regex::new(OPERATION_PATTERN).unwrap();
    let value_pattern = regex::Regex::new(VALUE_PATTERN).unwrap();
}
