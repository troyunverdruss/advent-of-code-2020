use std::collections::HashMap;

pub(crate) mod inputs;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
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
                map.insert(Point { x: x as i32, y: y as i32 }, String::from(entry));
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
                print!("{}", self.map.get(&Point  { x: x as i32, y: y as i32 }).unwrap())
            }
            println!();
        }
        println!()
    }
}
