use std::collections::HashMap;
use std::ops::Add;

pub(crate) mod inputs;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Point3 {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

impl Point3 {
    pub fn from2d(point: &Point) -> Point3 {
        Point3 { x: point.x, y: point.y, z: 0}
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
pub struct Point4 {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
    pub(crate) w: i32,
}

impl Point4 {
    pub fn from2d(point: &Point) -> Point4 {
        Point4 { x: point.x, y: point.y, z: 0, w: 0}
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GridData {
    pub(crate) map: HashMap<Point, char>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl GridData {
    pub fn parse_input(lines: Vec<String>) -> GridData {
        let mut map = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, entry) in line.chars().into_iter().enumerate() {
                map.insert(Point { x: x as i32, y: y as i32 }, entry);
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
