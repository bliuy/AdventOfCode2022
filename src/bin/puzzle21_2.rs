use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Add, Mul, Sub},
};

// Defining operation type
#[derive(Debug, Clone, Copy)]
enum OperationTypes {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equivalent,
}
#[derive(Debug, Clone)]
enum ValueTypes {
    Value(f64),
    Variable(f64, String),
    ValueAndVariable((f64, String), f64),
}

impl Add for ValueTypes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            ValueTypes::Value(lval) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::Value(lval + rval),
                ValueTypes::Variable(rcoef, rvar) => {
                    ValueTypes::ValueAndVariable((rcoef, rvar), lval)
                }
                ValueTypes::ValueAndVariable((rcoef, rvar), rval) => {
                    ValueTypes::ValueAndVariable((rcoef, rvar), lval + rval)
                }
            },
            ValueTypes::Variable(lcoef, lvar) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::ValueAndVariable((lcoef, lvar), rval),
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
            ValueTypes::ValueAndVariable((lcoef, lvar), lval) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::ValueAndVariable((lcoef, lvar), lval + rval),
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

impl Sub for ValueTypes {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            ValueTypes::Value(lval) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::Value(lval - rval),
                ValueTypes::Variable(rcoef, rvar) => {
                    ValueTypes::ValueAndVariable((-1.0 * rcoef, rvar), lval)
                }
                ValueTypes::ValueAndVariable((rcoef, rvar), rval) => {
                    ValueTypes::ValueAndVariable((-1.0 * rcoef, rvar), lval - rval)
                }
            },
            ValueTypes::Variable(lcoef, lvar) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::ValueAndVariable((lcoef, lvar), -1.0 * rval),
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
            ValueTypes::ValueAndVariable((lcoef, lvar), lval) => match rhs {
                ValueTypes::Value(rval) => {
                    ValueTypes::ValueAndVariable((-1.0 * lcoef, lvar), lval - rval)
                }
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

impl Mul for ValueTypes {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            ValueTypes::Value(lval) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::Value(lval * rval),
                ValueTypes::Variable(rcoef, rvar) => ValueTypes::Variable(lval * rcoef, rvar),
                ValueTypes::ValueAndVariable((rcoef, rvar), rval) => {
                    ValueTypes::ValueAndVariable((lval * rcoef, rvar), lval * rval)
                }
            },
            ValueTypes::Variable(lcoef, lvar) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::Variable(lcoef * rval, lvar),
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
            ValueTypes::ValueAndVariable((lcoef, lvar), lval) => match rhs {
                ValueTypes::Value(rval) => {
                    ValueTypes::ValueAndVariable((lcoef * rval, lvar), lval * rval)
                }
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

impl std::ops::Div for ValueTypes {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            ValueTypes::Value(lval) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::Value(lval / rval),
                ValueTypes::Variable(rcoef, rvar) => ValueTypes::Variable(lval / rcoef, rvar),
                ValueTypes::ValueAndVariable((rcoef, rvar), rval) => {
                    ValueTypes::ValueAndVariable((lval / rcoef, rvar), lval / rval)
                }
            },
            ValueTypes::Variable(lcoef, lvar) => match rhs {
                ValueTypes::Value(rval) => ValueTypes::Variable(lcoef / rval, lvar),
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
            ValueTypes::ValueAndVariable((lcoef, lvar), lval) => match rhs {
                ValueTypes::Value(rval) => {
                    ValueTypes::ValueAndVariable((lcoef / rval, lvar), lval / rval)
                }
                ValueTypes::Variable(_, _) => unreachable!(),
                ValueTypes::ValueAndVariable(_, _) => unreachable!(),
            },
        }
    }
}

#[derive(Debug)]
enum NodeTypes {
    Branch((String, String, OperationTypes)),
    Leaf(f64),
}
// Defining a node structure
#[derive(Debug)]
struct Node {
    name: String,
    operation_type: Option<OperationTypes>,
    node_value: Option<ValueTypes>,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn new(name: String) -> Self {
        Node {
            name: name,
            operation_type: None,
            node_value: None,
            lhs: None,
            rhs: None,
        }
    }
}

fn build_tree(node_name: String, relationships: &HashMap<String, NodeTypes>) -> Node {
    let mut node = Node::new(node_name.clone());
    match relationships.get(&node_name).unwrap() {
        NodeTypes::Branch((lhs, rhs, operation_type)) => {
            node.operation_type = Some(*operation_type);
            let lhs_node = build_tree(lhs.clone(), relationships);
            let rhs_node = build_tree(rhs.clone(), relationships);
            node.lhs = Some(Box::new(lhs_node));
            node.rhs = Some(Box::new(rhs_node));
        }
        NodeTypes::Leaf(val) => match node_name.as_str() {
            "humn" => {
                node.node_value = Some(ValueTypes::Variable(1.0, "humn".to_owned()));
            }
            _ => {
                node.node_value = Some(ValueTypes::Value(*val));
            }
        },
    };

    node
}

fn evaluate_tree(node: &Node) -> ValueTypes {
    match node.operation_type {
        Some(operation_type) => {
            let lhs_val: ValueTypes;
            let rhs_val: ValueTypes;
            match node.lhs.as_ref() {
                Some(lhs) => {
                    lhs_val = evaluate_tree(lhs.as_ref());
                }
                None => unreachable!(),
            }
            match node.rhs.as_ref() {
                Some(rhs) => {
                    rhs_val = evaluate_tree(rhs.as_ref());
                }
                None => unreachable!(),
            }
            // println!("LHS: {:?} {:?} RHS: {:?}", lhs_val, operation_type, rhs_val);
            match operation_type {
                OperationTypes::Plus => lhs_val + rhs_val,
                OperationTypes::Minus => lhs_val - rhs_val,
                OperationTypes::Multiply => lhs_val * rhs_val,
                OperationTypes::Divide => lhs_val / rhs_val,
                OperationTypes::Equivalent => {
                    println!("LHS: {:?}", lhs_val);
                    println!("RHS: {:?}", rhs_val);
                    ValueTypes::Value(0.0)
                }
            }
        }
        None => node.node_value.as_ref().unwrap().clone(),
    }
}

const OPERATION_PATTERN: &str = r"([a-z]+): ([a-z]+) ([+|\-|*|/|=]{1}) ([a-z]+)";
const VALUE_PATTERN: &str = r"([a-z]+): ([0-9]+)";

fn main() {
    // Reading inputs
    // let file = std::fs::File::open("inputs/examples/example21.txt").expect("Failed to open file.");
    let file = std::fs::File::open("inputs/alt/input21.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Compiling the regex pattern
    let operation_pattern = regex::Regex::new(OPERATION_PATTERN).unwrap();
    let value_pattern = regex::Regex::new(VALUE_PATTERN).unwrap();

    let mut tree_relationships: HashMap<String, NodeTypes> = HashMap::new();

    for rlines in reader.lines() {
        let line = rlines.unwrap();

        match value_pattern.captures(&line) {
            Some(matched) => {
                let val = NodeTypes::Leaf(matched.get(2).unwrap().as_str().parse().unwrap());
                let key = matched.get(1).unwrap().as_str().to_owned();
                tree_relationships.insert(key, val);
            }
            None => match operation_pattern.captures(&line) {
                Some(matched) => {
                    let key = matched.get(1).unwrap().as_str().to_owned();
                    let lhs = matched.get(2).unwrap().as_str().to_owned();
                    let rhs = matched.get(4).unwrap().as_str().to_owned();
                    let operator = matched.get(3).unwrap().as_str().to_owned();
                    let operator_type = if key == "root" {
                        OperationTypes::Equivalent
                    } else {
                        match operator.as_str() {
                            "+" => OperationTypes::Plus,
                            "-" => OperationTypes::Minus,
                            "*" => OperationTypes::Multiply,
                            "/" => OperationTypes::Divide,
                            _ => unreachable!(),
                        }
                    };

                    let val = NodeTypes::Branch((lhs, rhs, operator_type));
                    tree_relationships.insert(key, val);
                }
                None => unreachable!(),
            },
        }
    }

    // Building the operation tree
    let root_node = build_tree("root".to_owned(), &tree_relationships);

    // Evaluating the built tree
    evaluate_tree(&root_node);
}
