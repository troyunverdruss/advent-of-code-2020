use crate::day20::Orientation::{FlipBoth, FlipX, FlipY};
use crate::day20::Side::{Bottom, Left, Right, Top};
use crate::util::inputs::read_lines_split_by_double_newline;
use crate::util::{GridData, Point};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

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
                        edge: String::from(edge),
                    })
                } else if other_tile.edges_flip_x.contains(edge) {
                    count += 1;
                    let e = tiles_to_edges.entry(*id).or_insert(Vec::new());
                    e.push(EdgeMatch {
                        other_id: *other_id,
                        orientation: Orientation::FlipX,
                        edge: String::from(edge),
                    })
                } else if other_tile.edges_flip_y.contains(edge) {
                    count += 1;
                    let e = tiles_to_edges.entry(*id).or_insert(Vec::new());
                    e.push(EdgeMatch {
                        other_id: *other_id,
                        orientation: Orientation::FlipY,
                        edge: String::from(edge),
                    })
                } else if other_tile.edges_flip_both.contains(edge) {
                    count += 1;
                    let e = tiles_to_edges.entry(*id).or_insert(Vec::new());
                    e.push(EdgeMatch {
                        other_id: *other_id,
                        orientation: Orientation::FlipBoth,
                        edge: String::from(edge),
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

    let mut filled_map: HashMap<Point, char> = HashMap::new();
    let mut last_right = String::new();
    let mut last_bottom = String::new();
    let mut used_tiles: HashSet<i64> = HashSet::new();
    for y in 0..12 {
        for x in 0..12 {
            let offset = Point::new(x * 10, y * 10);
            println!("full grid entry: {:?}", offset);
            let here = Point::new(x, y);
            println!("here: {:?}", here);
            println!("last right: {}", last_right);
            // Get this thing started
            if x == 0 && y == 0 {
                let tile = lookup.get(start).unwrap();
                let edges = tiles_to_edges.get(start).unwrap();

                let edge1_match = edges.get(0).unwrap();
                let edge1_side = tile.which_side(&Orientation::Default, &edge1_match.edge);
                let edge2_match = edges.get(1).unwrap();
                let edge2_side = tile.which_side(&Orientation::Default, &edge2_match.edge);

                let mut flip_x = false;
                let mut flip_y = false;

                for edge in &[edge1_side, edge2_side] {
                    if edge == &Top {
                        flip_x = true
                    } else if edge == &Left {
                        flip_y = true
                    }
                }

                let flipped = flip_grid(&tile.grid_data.map, flip_x, flip_y);
                add_to_main_grid(&mut filled_map, &offset, &flipped);
                last_right = get_right(&flipped);
                last_bottom = get_bottom(&flipped);
                used_tiles.insert(tile.id);
                //
                // let orientation = getOrientation(flip_x, flip_y);
                //
                // let position = MapPosition {
                //     id: *start,
                //     orientation: orientation.clone(),
                //     rotation: 0,
                // };
                //
                println!("{:?} {:?}", &offset, tile.id);
                // debug_print(&tile.grid_data.map, &Orientation::Default, 0);
                print_grid(&filled_map);
                let i = 0;

            // map.insert(here, position);
            } else if x == 0 {
                // Otherwise look up if we're at the start of the row
                let up_pos = map.get(&Point::new(x, y - 1)).unwrap();
                let up_tile = lookup.get(&up_pos.id).unwrap();
                let up_edges = tiles_to_edges.get(&up_tile.id).unwrap();
                let bottom_edge = &up_tile.right(&up_pos.orientation);

                let edge_match = up_edges.iter().find(|e| e.edge == *bottom_edge).unwrap();

                let this_tile = lookup.get(&edge_match.other_id).unwrap();

                let required_orientation = get_required_orientation(bottom_edge, this_tile);
                let which_side = this_tile.which_side(&required_orientation, bottom_edge);
                let required_rotations = get_required_rotations(&which_side, &Top);

                map.insert(
                    here,
                    MapPosition {
                        id: this_tile.id,
                        orientation: required_orientation,
                        rotation: required_rotations,
                    },
                );
            } else {
                let mut possible_tile: Option<Tile> = None;
                let mut possible_grid: Option<HashMap<Point,char>> = None;
                let mut flip_x = false;
                let mut flip_y = false;
                for tile in &tiles {
                    if used_tiles.contains(&tile.id) {
                        continue;
                    }

                    let mut grid = tile.grid_data.map.clone();
                    let mut left = get_left(&grid);
                    let mut rots = 0;
                    let mut never_found = false;
                    if left == last_right {
                        possible_tile = Some(tile.clone());
                        possible_grid = Some(grid.clone());
                    }
                    while left != last_right {
                        grid = rotate_grid(&grid);
                        left = get_left(&grid);
                        rots += 1;
                        if rots > 4 {
                            grid = flip_grid(&grid, true, false);
                        }
                        if rots > 8 {
                            grid = flip_grid(&grid, true, true);
                        }
                        if rots > 12 {
                            grid = flip_grid(&grid, true, false);
                        }
                        if rots > 16 {
                            never_found = true;
                            break;
                        }
                    }
                    if !never_found {
                        possible_tile = Some(tile.clone());
                        possible_grid = Some(grid.clone());
                        break;
                    }

                    // if grid_has_edge(&tile.grid_data.map, &last_right) {
                    //     possible_tile = Some(tile.clone());
                    //     break;
                    // } else if grid_has_edge(
                    //     &flip_grid(&tile.grid_data.map, true, false),
                    //     &last_right,
                    // ) {
                    //     possible_tile = Some(tile.clone());
                    //     flip_x = true;
                    //     break;
                    // } else if grid_has_edge(
                    //     &flip_grid(&tile.grid_data.map, false, true),
                    //     &last_right,
                    // ) {
                    //     possible_tile = Some(tile.clone());
                    //     flip_y = true;
                    //     break;
                    // } else if grid_has_edge(
                    //     &flip_grid(&tile.grid_data.map, true, true),
                    //     &last_right,
                    // ) {
                    //     possible_tile = Some(tile.clone());
                    //     flip_x = true;
                    //     flip_y = true;
                    //     break;
                    // } else {
                    //     // no match
                    // }
                }
                let tile = possible_tile.unwrap();
                let grid = possible_grid.unwrap();
                // let mut grid = flip_grid(&tile.grid_data.map, flip_x, flip_y);

                print_grid(&grid);

                // let mut left = get_left(&grid);
                // let mut rots = 0;
                // while left != last_right {
                //     grid = rotate_grid(&grid);
                //     left = get_left(&grid);
                //     rots += 1;
                //     if rots > 4 {
                //         grid = flip_grid(&grid, true, false);
                //     }
                //     if rots > 8 {
                //         grid = flip_grid(&grid, false, true);
                //     }
                //     if rots > 12 {
                //         grid = flip_grid(&grid, true, true);
                //     }
                // }

                add_to_main_grid(&mut filled_map, &offset, &grid);
                last_right = get_right(&grid);
                used_tiles.insert(tile.id);
                println!("{:?} {:?}", &offset, tile.id);
                // debug_print(&tile.grid_data.map, &Orientation::Default, 0);
                print_grid(&filled_map);
                let i = 0;
                //     .iter()
                //     .filter(|t| !used_tiles.contains(&t.id))
                //     .filter(|t| {
                //
                //
                //
                //         t.edges.contains(&last_right)
                //                 || t.edges_flip_x.contains(&last_right)
                //                 || t.edges_flip_y.contains(&last_right)
                //                 || t.edges_flip_both.contains(&last_right))
                //     })
                //     .map(|t| t.clone())
                //     .collect();

                // lastly look left if we aren't at the start of a row
                // let left_pos = map.get(&Point::new(x - 1, y)).unwrap();
                // let left_tile = lookup.get(&left_pos.id).unwrap();
                // let left_edges = tiles_to_edges.get(&left_tile.id).unwrap();
                //
                // let right_edge =
                //     &left_tile.get_side(&Right, &left_pos.orientation, left_pos.rotation);
                // println!("right edge: {}", right_edge);
                //
                // let (_, this_tile) = lookup
                //     .iter()
                //     .filter(|e| e.1.has_edge(right_edge).is_some())
                //     .next()
                //     .unwrap();
                // let this_tile_orientation = this_tile.has_edge(right_edge).unwrap();
                // // let edge_match = left_edges.iter().find(|e| e.edge == *right_edge).unwrap();
                //
                // // let this_tile = lookup.get(&edge_match.other_id).unwrap();
                // let required_orientation = get_required_orientation(right_edge, this_tile);
                // let which_side = this_tile.which_side(&required_orientation, right_edge);
                // let required_rotations = get_required_rotations(&which_side, &Left);
                //
                // let position = MapPosition {
                //     id: this_tile.id,
                //     orientation: required_orientation.clone(),
                //     rotation: required_rotations,
                // };
                // println!("{:?} {:?}", here, position);
                // map.insert(here, position);
                // // debug_print(&this_tile.grid_data.map, &Orientation::Default, 0);
                // // debug_print(
                // //     &this_tile.grid_data.map,
                // //     &Orientation::Default,
                // //     required_rotations,
                // // );
                // debug_print(
                //     &this_tile.grid_data.map,
                //     &required_orientation,
                //     required_rotations,
                // );
            }
        }
    }
    let i = 0;
}

fn add_to_main_grid(filled_map: &mut HashMap<Point, char>, offset: &Point, grid: &HashMap<Point, char>) {
    let len = grid.iter().map(|e| e.0.x).max().unwrap();
    for y in 0..=len {
        for x in 0..=len {
            filled_map.insert(
                Point::new(offset.x + x, offset.y + y),
                grid.get(&Point::new(x, y)).unwrap().clone(),
            );
        }
    }
}

fn print_grid(grid: &HashMap<Point, char>) {
    let max_x = grid.iter().map(|e| e.0.x).max().unwrap();
    let max_y = grid.iter().map(|e| e.0.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            print!(
                "{}",
                grid.get(&Point {
                    x: x as i32,
                    y: y as i32
                })
                .unwrap()
            )
        }
        println!();
    }
}

fn debug_print(grid: &HashMap<Point, char>, orientation: &Orientation, rotations: u8) {
    let y_range: Vec<usize> = if orientation == &Orientation::Default || orientation == &FlipY {
        (0..=9).into_iter().collect()
    } else {
        (0..=9).rev().into_iter().collect()
    };

    let x_range: Vec<usize> = if orientation == &Orientation::Default || orientation == &FlipX {
        (0..=9).into_iter().collect()
    } else {
        (0..=9).rev().into_iter().collect()
    };

    let mut flipped_grid = HashMap::new();

    let mut new_y = 0;
    let mut new_x = 0;
    for y in y_range.clone() {
        new_x = 0;
        for x in x_range.clone() {
            flipped_grid.insert(
                Point::new(new_x, new_y),
                *grid.get(&Point::new(x as i32, y as i32)).unwrap(),
            );
            new_x += 1
        }
        new_y += 1;
    }

    let mut rotated_grid = flipped_grid.clone();
    for _ in 0..rotations {
        println!("rotate 1");
        rotated_grid = rotate_grid(&rotated_grid)
    }

    for y in 0..=9 {
        for x in 0..=9 {
            print!(
                "{}",
                rotated_grid
                    .get(&Point {
                        x: x as i32,
                        y: y as i32
                    })
                    .unwrap()
            )
        }
        println!();
    }
    println!()
}

fn rotate_grid(grid: &HashMap<Point, char>) -> HashMap<Point, char> {
    let mut rotated_grid = HashMap::new();

    let len = grid.iter().map(|e| e.0.x).max().unwrap();

    for y in 0..=len {
        for x in 0..=len {
            let pos = Point { x: len - y, y: x };
            rotated_grid.insert(pos, grid.get(&Point::new(x, y)).unwrap().clone());
        }
    }

    rotated_grid
}

fn flip_grid(grid: &HashMap<Point, char>, flip_x: bool, flip_y: bool) -> HashMap<Point, char> {
    let mut flipped_grid = HashMap::new();

    let len = grid.iter().map(|e| e.0.x).max().unwrap();

    for y in 0..=len {
        for x in 0..=len {
            let new_y = if flip_y { len - y } else { y };
            let new_x = if flip_x { len - x } else { x };

            let pos = Point { x: new_x, y: new_y };
            flipped_grid.insert(pos, grid.get(&Point::new(x, y)).unwrap().clone());
        }
    }

    flipped_grid
}

fn get_top(grid: &HashMap<Point, char>) -> String {
    let len = grid.iter().map(|e| e.0.x).max().unwrap();
    let mut top = vec![];
    for x in 0..=len {
        let pos = Point { x: x, y: 0 };
        top.push(grid.get(&pos).unwrap());
    }

    top.iter().join("")
}

fn get_bottom(grid: &HashMap<Point, char>) -> String {
    let len = grid.iter().map(|e| e.0.x).max().unwrap();
    let mut bottom = vec![];
    for x in 0..=len {
        let pos = Point { x: x, y: len };
        bottom.push(grid.get(&pos).unwrap());
    }

    bottom.iter().join("")
}

fn get_left(grid: &HashMap<Point, char>) -> String {
    let len = grid.iter().map(|e| e.0.x).max().unwrap();
    let mut left = vec![];
    for y in 0..=len {
        let pos = Point { x: 0, y: y };
        left.push(grid.get(&pos).unwrap());
    }

    left.iter().join("")
}

fn get_right(grid: &HashMap<Point, char>) -> String {
    let len = grid.iter().map(|e| e.0.x).max().unwrap();
    let mut right = vec![];
    for y in 0..=len {
        let pos = Point { x: len, y: y };
        right.push(grid.get(&pos).unwrap());
    }

    right.iter().join("")
}

fn grid_has_edge(grid: &HashMap<Point, char>, edge: &String) -> bool {
    let top = get_top(grid) == *edge;
    let bottom = get_bottom(grid) == *edge;
    let left = get_left(grid) == *edge;
    let right = get_right(grid) == *edge;

    top || bottom || left || right
}

fn get_required_rotations(current_side: &Side, target_side: &Side) -> u8 {
    let order = vec![Top, Right, Bottom, Left];
    let current_index = order
        .iter()
        .position(|s| s == current_side)
        .expect("has to be there");
    let target_index = order
        .iter()
        .position(|s| s == target_side)
        .expect("has to be there");

    if current_index == target_index {
        0
    } else if (current_index + 1) % 4 == target_index {
        1
    } else if (current_index + 2) % 4 == target_index {
        2
    } else if (current_index + 3) % 4 == target_index {
        3
    } else {
        unreachable!()
    }
}

fn get_required_orientation(edge: &String, tile: &Tile) -> Orientation {
    if tile.edges.contains(edge) {
        Orientation::Default
    } else if tile.edges_flip_x.contains(edge) {
        FlipX
    } else if tile.edges_flip_y.contains(edge) {
        FlipY
    } else
    //if tile.edges_flip_both.contains(edge) {
    //FlipBoth
    //} else
    {
        unreachable!()
    }
}

fn getOrientation(flip_x: bool, flip_y: bool) -> Orientation {
    if flip_x && flip_y {
        Orientation::FlipBoth
    } else if flip_x {
        Orientation::FlipX
    } else if flip_y {
        Orientation::FlipY
    } else {
        Orientation::Default
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Tile {
    id: i64,
    grid_data: GridData,
    edges: HashSet<String>,
    edges_flip_x: HashSet<String>,
    edges_flip_y: HashSet<String>,
    edges_flip_both: HashSet<String>,
    top: String,
    bottom: String,
    left: String,
    right: String,
}

impl Tile {
    fn top(&self, orientation: &Orientation) -> String {
        match orientation {
            Orientation::Default => self.top.clone(),
            Orientation::FlipX => self.bottom.clone(),
            Orientation::FlipY => self.top.chars().rev().join(""),
            Orientation::FlipBoth => self.bottom.chars().rev().join(""),
        }
    }

    fn bottom(&self, orientation: &Orientation) -> String {
        match orientation {
            Orientation::Default => self.bottom.clone(),
            Orientation::FlipX => self.top.clone(),
            Orientation::FlipY => self.bottom.chars().rev().join(""),
            Orientation::FlipBoth => self.top.chars().rev().join(""),
        }
    }

    fn left(&self, orientation: &Orientation) -> String {
        match orientation {
            Orientation::Default => self.left.clone(),
            Orientation::FlipX => self.left.chars().rev().join(""),
            Orientation::FlipY => self.right.clone(),
            Orientation::FlipBoth => self.right.chars().rev().join(""),
        }
    }

    fn right(&self, orientation: &Orientation) -> String {
        match orientation {
            Orientation::Default => self.right.clone(),
            Orientation::FlipX => self.right.chars().rev().join(""),
            Orientation::FlipY => self.left.clone(),
            Orientation::FlipBoth => self.left.chars().rev().join(""),
        }
    }

    fn get_side(&self, target_side: &Side, orientation: &Orientation, rotations: u8) -> String {
        let order = vec![Top, Right, Bottom, Left];
        let current_index = order
            .iter()
            .position(|s| s == target_side)
            .expect("has to be there");

        let target_side = order
            .get(((current_index as i16 - rotations as i16) % 4).abs() as usize)
            .unwrap();

        match target_side {
            Top => self.top(&orientation),
            Bottom => self.bottom(&orientation),
            Left => self.left(&orientation),
            Right => self.right(&orientation),
        }
    }

    fn which_side(&self, orientation: &Orientation, side: &str) -> Side {
        if self.top(orientation) == side {
            Top
        } else if self.bottom(orientation) == side {
            Bottom
        } else if self.left(orientation) == side {
            Left
        } else if self.right(orientation) == side {
            Right
        } else {
            unreachable!()
        }
    }

    fn has_edge(&self, edge: &String) -> Option<Orientation> {
        if self.edges.contains(edge) {
            Some(Orientation::Default)
        } else if self.edges_flip_x.contains(edge) {
            Some(Orientation::FlipX)
        } else if self.edges_flip_y.contains(edge) {
            Some(FlipY)
        } else {
            None
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
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct EdgeMatch {
    other_id: i64,
    orientation: Orientation,
    edge: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct MapPosition {
    id: i64,
    orientation: Orientation,
    rotation: u8,
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
        edges_flip_y: edged_flip_y,
        edges_flip_both,
        top,
        bottom,
        left,
        right,
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::{
        debug_print, flip_grid, get_bottom, get_left, get_right, get_top, print_grid, rotate_grid,
    };
    use crate::util::Point;
    use std::collections::HashMap;

    #[test]
    fn rotate() {
        let mut grid = HashMap::new();
        let chars = "abcdefghi".chars().collect::<Vec<char>>();
        let mut char_idx = 0;
        for y in 0..3 {
            for x in 0..3 {
                grid.insert(Point::new(x, y), *chars.get(char_idx).unwrap());
                char_idx += 1;
            }
        }
        print_grid(&grid);

        let mut rotated_grid = grid.clone();
        for r in 1..4 {
            println!("rotate {}", r);
            rotated_grid = rotate_grid(&rotated_grid);
            print_grid(&rotated_grid);
        }

        println!();

        print_grid(&grid);
        println!("no flip");
        print_grid(&flip_grid(&grid, false, false));
        println!("x flip");
        print_grid(&flip_grid(&grid, true, false));
        println!("y flip");
        print_grid(&flip_grid(&grid, false, true));
        println!("x y flip");
        print_grid(&flip_grid(&grid, true, true));

        println!();
        println!("top: {}", get_top(&grid));
        println!("bottom: {}", get_bottom(&grid));
        println!("left: {}", get_left(&grid));
        println!("right: {}", get_right(&grid));
    }
}
