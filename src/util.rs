use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(day: i8) -> Vec<String> {
    let path = format!("inputs/input{:02}.txt", day);
    let file = File::open(path).expect("Unable to open input file");

    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.expect("No line read"))
    }

    lines
}
