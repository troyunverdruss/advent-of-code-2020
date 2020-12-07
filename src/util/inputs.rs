use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn day_input(day: i8) -> Vec<String> {
    let path = format!("inputs/input{:02}.txt", day);
    read_lines(path)
}

pub fn read_lines(path: String) -> Vec<String> {
    let file = File::open(path).expect("Unable to open input file");

    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>()
}

pub fn read_lines_split_by_double_newline(day: i8) -> Vec<String> {
    read_lines_to_string(day).split("\n\n").map(String::from).collect()
}

pub fn read_lines_to_string(day: i8) -> String {
    let path = format!("inputs/input{:02}.txt", day);
    let file = File::open(path).expect("Unable to open input file");

    let mut reader = BufReader::new(file);
    let mut line: String = String::new();

    reader.read_to_string(&mut line)
        .expect("Reading file to line failed");

    line
}
