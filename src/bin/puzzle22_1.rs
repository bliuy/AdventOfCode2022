use regex;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    ForceField,
    Wall,
    Open,
}
#[derive(Debug, Clone, Copy)]
enum Move {
    Steps(i32),
    ClockwiseTurn,
    AntiClockwiseTurn,
}

fn main() {
    // Reading the file
    let file = std::fs::File::open("inputs/input22_map.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Constructing the map
    let mut map = Vec::new();
    for rline in reader.lines() {
        let row = rline
            .unwrap()
            .chars()
            .map(|x| match x {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                ' ' => Tile::ForceField,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        map.push(row);
    }

    // Constructing the directions
    let file = std::fs::File::open("inputs/input22_directions.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);
    let raw_string = reader.lines().nth(0).unwrap().unwrap();

    // Parsing the raw direction string
    let num_regex = regex::Regex::new("([0-9]+)").unwrap();
    let dir_regex = regex::Regex::new("([L|R])").unwrap();

    // Constructing the directions list
    let mut num_matches = num_regex.find_iter(&raw_string);
    let mut dir_matches = dir_regex.find_iter(&raw_string);

    let mut moves: Vec<Move> = Vec::new();
    loop {
        let num_match = num_matches.next();
        let dir_match = dir_matches.next();
        if (num_match == None) && (dir_match == None) {
            break;
        }

        match num_match {
            Some(matched) => {
                let num = matched.as_str().parse::<i32>().unwrap();
                moves.push(Move::Steps(num));
            }
            None => {}
        }

        match dir_match {
            Some(matched) => {
                let dir = match matched.as_str() {
                    "L" => Move::AntiClockwiseTurn,
                    "R" => Move::ClockwiseTurn,
                    _ => unreachable!(),
                };
                moves.push(dir)
            }
            None => {}
        }
    }
    println!("{:?}", moves);
}
