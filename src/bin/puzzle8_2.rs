use std::{
    fs::read,
    io::{BufRead, Read},
};

fn scenic_score(
    arr: &Vec<Vec<u32>>,
    val: u32,
    pos: (usize, usize),
    max_pos: (usize, usize),
) -> u32 {
    let mut flag_visible: bool = false;

    // Getting the current value
    let (mut x, mut y) = pos;
    let (x_max, y_max) = max_pos;
    let tree = val;
    let mut score = 1;

    // Edges
    if (x == 0) | (x == x_max) | (y == 0) | (y == y_max) {
        return 0;
    }

    // Traversing each direction
    let mut current;

    // Traversing top direction
    (x, y) = pos;
    current = 0;
    while x > 0 {
        x = x - 1; // Moves to the new tree
        current += 1; // Adds an additional tree to the counter
        let other = arr.get(x).unwrap().get(y).unwrap(); // Evaluating the height of the other tree

        // Reaches a other tree that blocks the current tree - Break loop and update current score.
        if other >= &tree {
            score *= current;
            break;
        }

        // Reaches the edge - Update current score.
        if x == 0 {
            score *= current;
        }
    }

    // Traversing bottom direction
    (x, y) = pos;
    current = 0;
    while x < x_max {
        x = x + 1; // Moves to the new tree
        current += 1; // Adds an additional tree to the counter
        let other = arr.get(x).unwrap().get(y).unwrap(); // Evaluating the height of the other tree

        // Reaches a other tree that blocks the current tree - Break loop and update current score.
        if other >= &tree {
            score *= current;
            break;
        }

        // Reaches the edge - Update current score.
        if x == x_max {
            score *= current;
        }
    }

    // Traversing left direction
    (x, y) = pos;
    current = 0;
    while y > 0 {
        y = y - 1; // Moves to the new tree
        current += 1; // Adds an additional tree to the counter
        let other = arr.get(x).unwrap().get(y).unwrap(); // Evaluating the height of the other tree

        // Reaches a other tree that blocks the current tree - Break loop and update current score.
        if other >= &tree {
            score *= current;
            break;
        }

        // Reaches the edge - Update current score.
        if y == 0 {
            score *= current;
        }
    }

    // Traversing right direction
    (x, y) = pos;
    current = 0;
    while y < y_max {
        y = y + 1; // Moves to the new tree
        current += 1; // Adds an additional tree to the counter
        let other = arr.get(x).unwrap().get(y).unwrap(); // Evaluating the height of the other tree

        // Reaches a other tree that blocks the current tree - Break loop and update current score.
        if other >= &tree {
            score *= current;
            break;
        }

        // Reaches the edge - Update current score.
        if y == y_max {
            score *= current;
        }
    }

    score
}

fn main() {
    // Reading the file
    let file = std::fs::File::open("inputs/input8.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    let mut matrix = Vec::new();

    // Filling up the matrix
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        line.chars().into_iter().for_each(|c| {
            let int = c.to_digit(10).unwrap();
            row.push(int)
        });
        matrix.push(row)
    }

    // Iterating through each row
    let mut unblocked_count: i32 = 0;
    let i_max = matrix.len() - 1;
    let j_max = matrix[0].len() - 1;
    let mut max_score = 0;

    matrix.iter().enumerate().for_each(|(i, row)| {
        row.into_iter().enumerate().for_each(|(j, val)| {
            let score = scenic_score(&matrix, *val, (i, j), (i_max, j_max));
            if score > max_score {
                max_score = score;
            }
        })
    });

    println!("Result: {}", max_score);
}
