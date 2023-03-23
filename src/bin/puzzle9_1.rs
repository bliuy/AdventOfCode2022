use std::{collections::HashSet, io::BufRead};

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

type Position = (i32, i32);

struct Head {
    pos: Position,
    prev_pos: Position,
}

struct Tail {
    pos: Position,
    visited: HashSet<Position>,
}

impl Head {
    fn new() -> Self {
        Head {
            pos: (0, 0),
            prev_pos: (0, 0),
        }
    }

    fn move_dir(&mut self, direction: &Direction) {
        self.prev_pos = self.pos; // Assigning the current position to the previous position prior to a move.
        match direction {
            Direction::U => self.pos.1 += 1,
            Direction::D => self.pos.1 += -1,
            Direction::L => self.pos.0 += -1,
            Direction::R => self.pos.0 += 1,
        }
    }
}

impl Tail {
    fn new() -> Self {
        let mut tail = Tail {
            pos: (0, 0),
            visited: HashSet::new(),
        };
        tail.visited.insert((0, 0)); // Including the initial position
        tail
    }

    fn follow_head(&mut self, head: &Head) {
        // Checking positional delta
        let (x_delta, y_delta) = (head.pos.0 - self.pos.0, head.pos.1 - self.pos.1);
        if (x_delta.abs() <= 1) & (y_delta.abs() <= 1) {
            return; // Tail is in contact with the head
        } else {
            self.pos = head.prev_pos; // Moving to the position previously occupied by the head.
        }

        // Updating the visited field
        self.visited.insert(self.pos);
    }
}

fn main() {
    // Reading the file
    let file = std::fs::File::open("inputs/input9.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    let pattern = regex::Regex::new("([A-Z]{1}) ([0-9]+)").unwrap();
    let mut head = Head::new();
    let mut tail = Tail::new();
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
            // println!("Head moves: {:?}", &direction);
            head.move_dir(&direction);
            tail.follow_head(&head);
            // println!("Head: {:?}, Tail: {:?}", &head.pos, &tail.pos);
            i += 1;
        }
    }

    println!("Result: {}", &tail.visited.len());
}
