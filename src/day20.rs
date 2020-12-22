use crate::util::inputs::read_lines_split_by_double_newline;
use crate::util::{GridData, Point};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let tiles = read_lines_split_by_double_newline(20);
    let tiles: Vec<Tile> = tiles
        .iter()
        .map(|l| parse_tile(l.split('\n').map(String::from).collect()))
        .collect();

    let mut lookup: HashMap<i64, Tile> = HashMap::new();
    for t in &tiles {
        lookup.insert(t.id, t.clone());
    }

    let mut tiles_to_edges: HashMap<i64, Vec<EdgeMatch>> = HashMap::new();
    let mut part1_product = 1;
    let mut sum = 0;
    let ids = tiles.iter().map(|t| t.id).collect::<Vec<i64>>();
    for id in &ids {
        let mut count = 0;
        for edge in &lookup.get(id).unwrap().edges {
            for other_id in &ids {
                if id == other_id {
                    continue;
                }
                let other_tile = lookup.get(other_id).unwrap();
                if other_tile.edges.contains(edge) {
                    count += 1;
                    let e = tiles_to_edges.entry(*id).or_insert(Vec::new());
                    e.push(EdgeMatch {
                        other_id: *other_id,
                        orientation: Orientation::Default,
                    })
                } else if other_tile.edges_flip_x.contains(edge) {
                    count += 1;
                    let e = tiles_to_edges.entry(*id).or_insert(Vec::new());
                    e.push(EdgeMatch {
                        other_id: *other_id,
                        orientation: Orientation::FlipX,
                    })
                } else if other_tile.edged_flip_y.contains(edge) {
                    count += 1;
                    let e = tiles_to_edges.entry(*id).or_insert(Vec::new());
                    e.push(EdgeMatch {
                        other_id: *other_id,
                        orientation: Orientation::FlipY,
                    })
                } else if other_tile.edges_flip_both.contains(edge) {
                    count += 1;
                    let e = tiles_to_edges.entry(*id).or_insert(Vec::new());
                    e.push(EdgeMatch {
                        other_id: *other_id,
                        orientation: Orientation::FlipBoth,
                    })
                } else {
                    count += 0;
                    // perimeter edge
                }
            }
        }

        if count == 2 {
            part1_product *= id
        }
        sum += count;
    }
    println!("Part 1: {}", part1_product);
    println!("  sums: {}", sum);

    let start = tiles_to_edges
        .iter()
        .filter(|e| e.1.len() == 2)
        .next()
        .unwrap()
        .0;

    let mut map: HashMap<Point, MapPosition> = HashMap::new();

    for y in 0..12 {
        for x in 0..12 {
            let here = Point::new(x, y);
            // Get this thing started
            if x == 0 && y == 0 {
                map.insert(
                    Point::new(0, 0),
                    MapPosition {
                        id: *start,
                        orientation: Orientation::Default,
                    },
                );
            } else if x == 0 {
                // Otherwise look up if we're at the start of the row
                let up_tile = map.get(&Point::new(x, y - 1)).unwrap();
            } else {
                // lastly look left if we aren't at the start of a row
                let left_tile = map.get(&Point::new(x - 1, y)).unwrap();
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Tile {
    id: i64,
    grid_data: GridData,
    edges: HashSet<String>,
    edges_flip_x: HashSet<String>,
    edged_flip_y: HashSet<String>,
    edges_flip_both: HashSet<String>,
    top: String,
    bottom: String,
    left: String,
    right: String,
}

impl Tile {
    fn top(&self, orientation: Orientation) -> String {
        match orientation {
            Orientation::Default => self.top.clone(),
            Orientation::FlipX => self.bottom.clone(),
            Orientation::FlipY => self.top.clone(),
            Orientation::FlipBoth => self.bottom.clone(),
        }
    }

    fn bottom(&self, orientation: Orientation) -> String {
        match orientation {
            Orientation::Default => self.bottom.clone(),
            Orientation::FlipX => self.top.clone(),
            Orientation::FlipY => self.bottom.clone(),
            Orientation::FlipBoth => self.top.clone(),
        }
    }

    fn left(&self, orientation: Orientation) -> String {
        match orientation {
            Orientation::Default => self.left.clone(),
            Orientation::FlipX => self.left.clone(),
            Orientation::FlipY => self.right.clone(),
            Orientation::FlipBoth => self.right.clone(),
        }
    }

    fn right(&self, orientation: Orientation) -> String {
        match orientation {
            Orientation::Default => self.right.clone(),
            Orientation::FlipX => self.right.clone(),
            Orientation::FlipY => self.left.clone(),
            Orientation::FlipBoth => self.left.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Orientation {
    Default,
    FlipX,
    FlipY,
    FlipBoth,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct EdgeMatch {
    other_id: i64,
    orientation: Orientation,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct MapPosition {
    id: i64,
    orientation: Orientation,
}

fn parse_tile(lines: Vec<String>) -> Tile {
    lazy_static! {
        static ref RE_TILE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    }
    let captures = RE_TILE.captures(lines.get(0).unwrap());
    let groups = captures.unwrap();
    let id: i64 = groups
        .get(1)
        .map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());

    let grid = lines[1..]
        .iter()
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();
    let grid_data = GridData::parse_input(grid);

    let mut edges: HashSet<String> = HashSet::new();
    let mut edges_flip_x: HashSet<String> = HashSet::new();
    let mut edged_flip_y: HashSet<String> = HashSet::new();
    let mut edges_flip_both: HashSet<String> = HashSet::new();

    let mut top = vec![];
    for x in 0..grid_data.cols {
        top.push(*grid_data.map.get(&Point { x: x as i32, y: 0 }).unwrap())
    }
    let top: String = top
        .iter()
        .map(|c| String::from(*c))
        .collect::<Vec<String>>()
        .join("");
    edges.insert(top.clone());
    edges_flip_x.insert(top.clone());
    edged_flip_y.insert(top.chars().rev().join(""));
    edges_flip_both.insert(top.chars().rev().join(""));

    let mut bottom = vec![];
    for x in 0..grid_data.cols {
        bottom.push(
            *grid_data
                .map
                .get(&Point {
                    x: x as i32,
                    y: (grid_data.rows - 1) as i32,
                })
                .unwrap(),
        )
    }
    let bottom: String = bottom
        .iter()
        .map(|c| String::from(*c))
        .collect::<Vec<String>>()
        .join("");
    edges.insert(bottom.clone());
    edges_flip_x.insert(bottom.clone());
    edged_flip_y.insert(bottom.chars().rev().join(""));
    edges_flip_both.insert(bottom.chars().rev().join(""));

    let mut left = vec![];
    for y in 0..grid_data.rows {
        left.push(
            *grid_data
                .map
                .get(&Point {
                    x: 0 as i32,
                    y: y as i32,
                })
                .unwrap(),
        )
    }
    let left: String = left
        .iter()
        .map(|c| String::from(*c))
        .collect::<Vec<String>>()
        .join("");
    edges.insert(left.clone());
    edges_flip_x.insert(left.chars().rev().join(""));
    edged_flip_y.insert(left.clone());
    edges_flip_both.insert(left.chars().rev().join(""));

    let mut right = vec![];
    for y in 0..grid_data.rows {
        right.push(
            *grid_data
                .map
                .get(&Point {
                    x: (grid_data.cols - 1) as i32,
                    y: y as i32,
                })
                .unwrap(),
        )
    }
    let right: String = right
        .iter()
        .map(|c| String::from(*c))
        .collect::<Vec<String>>()
        .join("");
    edges.insert(right.clone());
    edges_flip_x.insert(right.chars().rev().join(""));
    edged_flip_y.insert(right.clone());
    edges_flip_both.insert(right.chars().rev().join(""));

    Tile {
        id: id,
        grid_data,
        edges,
        edges_flip_x,
        edged_flip_y,
        edges_flip_both,
    }
}
