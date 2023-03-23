use std::{
    fs::read,
    io::{BufRead, Read},
};

fn is_visible(arr: &Vec<Vec<u32>>, val: u32, pos: (usize, usize), max_pos: (usize, usize)) -> bool {
    let mut flag_visible: bool = false;

    // Getting the current value
    let (mut x, mut y) = pos;
    let (x_max, y_max) = max_pos;
    let tree = val;

    // Edges
    if (x == 0) | (x == x_max) | (y == 0) | (y == y_max) {
        flag_visible = true;
        return flag_visible;
    }

    // Traversing each direction

    // Traversing top direction
    (x, y) = pos;
    while x > 0 {
        x = x - 1;
        let other = arr.get(x).unwrap().get(y).unwrap();
        if other >= &tree {
            break;
        }
        if x == 0 {
            flag_visible = true;
            return flag_visible;
        }
    }

    // Traversing bottom direction
    (x, y) = pos;
    while x < x_max {
        x = x + 1;
        let other = arr.get(x).unwrap().get(y).unwrap();
        if other >= &tree {
            break;
        }
        if x == x_max {
            flag_visible = true;
            return flag_visible;
        }
    }

    // Traversing left direction
    (x, y) = pos;
    while y > 0 {
        y = y - 1;
        let other = arr.get(x).unwrap().get(y).unwrap();
        if other >= &tree {
            break;
        }
        if y == 0 {
            flag_visible = true;
            return flag_visible;
        }
    }

    // Traversing right direction
    (x, y) = pos;
    while y < y_max {
        y = y + 1;
        let other = arr.get(x).unwrap().get(y).unwrap();
        if other >= &tree {
            break;
        }
        if y == y_max {
            flag_visible = true;
            return flag_visible;
        }
    }

    flag_visible
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

    matrix.iter().enumerate().for_each(|(i, row)| {
        row.into_iter().enumerate().for_each(|(j, val)| {
            if is_visible(&matrix, *val, (i, j), (i_max, j_max)) {
                unblocked_count += 1
            }
        })
    });

    println!("Result: {}", unblocked_count);

    // let mut i = 0;
    // let mut j = 0;

    // loop {
    //     // Accessing the element
    //     if (i == 0) | (i == i_max) | (j == 0) | (j == j_max) {
    //         // These trees are at the perimeter - have at least 1 side that is open.
    //         unblocked_count += 1
    //     } else {
    //         let tree = matrix.get(i).unwrap().get(j).unwrap();
    //         let top_tree = matrix.get(i - 1).unwrap().get(j).unwrap();
    //         let bottom_tree = matrix.get(i + 1).unwrap().get(j).unwrap();
    //         let left_tree = matrix.get(i).unwrap().get(j - 1).unwrap();
    //         let right_tree = matrix.get(i).unwrap().get(j + 1).unwrap();
    //     }

    //     // Checking if the next row needs to be accessed.
    // }
}
