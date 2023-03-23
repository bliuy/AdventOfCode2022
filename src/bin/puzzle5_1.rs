use std::io::BufRead;

fn main() {
    // Reading the file
    let file = std::fs::File::open("inputs/input5.txt").expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    // Hardcoding the configuration
    let mut arrangement = Vec::with_capacity(9);
    arrangement.push(
        "bgsc"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "tmwhjnvg"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "mqs"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "bsltwnm"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "jzftvgwp"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "ctbgqhs"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "tjpbw"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "gdczftqm"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );
    arrangement.push(
        "nshbpf"
            .chars()
            .into_iter()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<_>>(),
    );

    // Compiling the regex used for matching the pattern
    let pattern = regex::Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    for line in reader.lines().skip(10) {
        let line = line.unwrap();
        let captures = pattern.captures(&line).unwrap(); // Should always be able to capture the intended groups.
        let (mut count, starting, ending) = (
            captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );

        while count > 0 {
            let val: char = *&mut arrangement[starting - 1].pop().unwrap();
            let _ = &mut arrangement[ending - 1].push(val);
            count += -1;
        }
    }

    for q in arrangement {
        println!("{:?}", q.last().unwrap());
    }
}
