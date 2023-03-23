use std::{collections::HashSet, error::Error, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    // Reading the file
    let mut file = std::fs::File::open("inputs/input6.txt").expect("Failed to open file.");
    let mut signal = String::new();
    file.read_to_string(&mut signal)?;
    let chars = signal.chars().collect::<Vec<_>>();
    let (mut i, mut j) = (0, 4);
    while j < chars.len() {
        let result = is_distinct(chars.get(i..j).unwrap());
        if result == true {
            println!("{:?}", chars.get(i..j));
            println!("Result: {}", j);
        }
        i += 1;
        j += 1;
    }

    Ok(())
}

pub fn is_distinct(arr: &[char]) -> bool {
    let distinct = arr.iter().collect::<HashSet<_>>();
    if distinct.len() < arr.len() {
        return false;
    }
    true
}
