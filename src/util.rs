use std::collections::HashMap;
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

pub fn read_lines_to_string(day: i8) -> String {
    let path = format!("inputs/input{:02}.txt", day);
    let file = File::open(path).expect("Unable to open input file");

    let mut reader = BufReader::new(file);
    let mut line: String = String::new();

    reader.read_to_string(&mut line);

    line
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

pub struct GridData {
    pub(crate) map: HashMap<Point, String>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl GridData {
    pub fn parse_input(lines: Vec<String>) -> GridData {
        let mut map = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, entry) in line.chars().into_iter().enumerate() {
                map.insert(Point { x, y }, String::from(entry));
                // println! {"{:?}{}", Point { x, y }, entry.to_owned()}
            }
        }

        GridData {
            map,
            rows: lines.len(),
            cols: lines.get(0).unwrap().len(),
        }
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                print!("{}", self.map.get(&Point { x, y }).unwrap())
            }
            println!();
        }
    }
}
