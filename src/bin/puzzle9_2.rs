use std::{collections::HashSet, io::BufRead};

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

type Position = (i32, i32);

struct Rope {
    body: [Knot; 10],
    visited: HashSet<Position>,
}

impl Rope {
    fn new() -> Self {
        let knot = Knot::new();
        let body = [knot; 10];
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        Rope {
            body: body,
            visited: visited,
        }
    }

    fn move_rope(&mut self, direction: &Direction) {
        // Moving the head section - Will always move
        let head = self.body.get_mut(0).unwrap();
        match direction {
            Direction::U => head.pos.1 += 1,
            Direction::D => head.pos.1 += -1,
            Direction::L => head.pos.0 += -1,
            Direction::R => head.pos.0 += 1,
        }

        // Iterating through each window and updating them
        let mut i = 0; // Represents the splitting point
        let length = self.body.len();
        while i < length - 1 {
            let (f, b) = self.body.split_at_mut(i + 1);
            let front = f.last().unwrap();
            let back = b.first_mut().unwrap();
            if !back.is_contacting(front) {
                back.follow(&front);
            }
            i += 1;
        }

        // Updating the visited positions
        self.visited.insert(self.body.last().unwrap().pos);
    }
}

#[derive(Debug, Clone, Copy)]
struct Knot {
    pos: Position,
}

impl Knot {
    fn new() -> Self {
        Knot { pos: (0, 0) }
    }

    fn is_contacting(&self, front: &Knot) -> bool {
        let (x_delta, y_delta) = (front.pos.0 - self.pos.0, front.pos.1 - self.pos.1);
        if (x_delta.abs() <= 1) & (y_delta.abs() <= 1) {
            true // Tail is in contact with the head.
        } else {
            false
        }
    }

    fn follow(&mut self, front: &Knot) {
        // Calculating the deltas
        let (x_delta, y_delta) = (front.pos.0 - self.pos.0, front.pos.1 - self.pos.1);

        // Moving on the x-axis
        match x_delta {
            1.. => self.pos.0 += 1,
            0 => {} // No movement required, since the knots are on the same row.
            ..=-1 => self.pos.0 += -1,
        }

        // Moving on the y-axis
        match y_delta {
            1.. => self.pos.1 += 1,
            0 => {} // No movement required, since the knots are on the same col.
            ..=-1 => self.pos.1 += -1,
        }
    }
}

fn main() {
    // Reading the file
    let file = std::fs::File::open("inputs/input9.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);
    let mut rope = Rope::new();
    let pattern = regex::Regex::new("([A-Z]{1}) ([0-9]+)").unwrap();
    for rline in reader.lines() {
        let line = rline.unwrap();
        let groups = pattern.captures(&line).unwrap();
        let direction = match groups.get(1).unwrap().as_str() {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => panic!(),
        };
        let step_count = groups.get(2).unwrap().as_str().parse::<i32>().unwrap();

        // Completing the steps for this round
        let mut i = 0;
        while i < step_count {
            rope.move_rope(&direction);
            i += 1;
        }
    }

    println!("Result: {}", rope.visited.len())
}
