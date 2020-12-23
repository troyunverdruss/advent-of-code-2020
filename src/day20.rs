use crate::day20::Orientation::{FlipBoth, FlipX, FlipY};
use crate::day20::Side::{Bottom, Left, Right, Top};
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

    // part 2

    let start = &(3359 as i64);

    let mut filled_map: HashMap<Point, char> = HashMap::new();
    let mut last_right = String::new();
    let mut last_bottom = String::new();
    let mut used_tiles: HashSet<i64> = HashSet::new();
    for y in 0..12 {
        for x in 0..12 {
            // println!("Used entries: {}", used_tiles.len());
            // println!("Used entries: {:?}", used_tiles);
            let offset = Point::new(x * 10, y * 10);
            // println!("full grid entry: {:?}", offset);
            // println!("here: {:?}", here);
            // println!("last right: {}", last_right);
            // println!("last bottom: {}", last_bottom);
            // Get this thing started
            if x == 0 && y == 0 {
                let tile = lookup.get(start).unwrap();
                let edges = tiles_to_edges.get(start).unwrap();

                let edge1_match = edges.get(0).unwrap();
                let edge1_side = which_side_is_this(&tile.grid_data.map, &edge1_match.edge);
                let edge2_match = edges.get(1).unwrap();
                let edge2_side = which_side_is_this(&tile.grid_data.map, &edge2_match.edge);

                let mut flip_x = false;
                let mut flip_y = false;

                if edge1_side == Left || edge2_side == Left {
                    flip_y = true;
                }
                if edge1_side == Top || edge2_side == Top {
                    flip_x = true;
                }

                let flipped = flip_grid(&tile.grid_data.map, flip_x, flip_y);
                add_to_main_grid(&mut filled_map, &offset, &flipped);

                last_right = get_right(&flipped);
                last_bottom = get_bottom(&flipped);

                used_tiles.insert(tile.id);
            // println!("{:?} {:?}", &offset, tile.id);
            // print_grid(&filled_map);
            // let i = 0;
            } else if x == 0 {
                // Otherwise look up if we're at the start of the row
                let mut this_tile = None;
                let mut this_grid = None;
                for tile in &tiles {
                    if used_tiles.contains(&tile.id) {
                        // println!("Skipping {}", tile.id);
                        continue;
                    }
                    let mut grid = tile.grid_data.map.clone();
                    for i in 0..4 {
                        let top = get_top(&grid);
                        if top == last_bottom {
                            // println!("(l) found matching left edge: {} {}", tile.id, top);
                            this_tile = Some(tile.clone());
                            this_grid = Some(grid.clone())
                        }
                        grid = rotate_grid(&grid);
                    }
                    let mut grid = tile.grid_data.map.clone();
                    grid = flip_grid(&grid, true, false);
                    for i in 0..4 {
                        let top = get_top(&grid);
                        if top == last_bottom {
                            // println!("(f) found matching left edge: {} {}", tile.id, top);
                            this_tile = Some(tile.clone());
                            this_grid = Some(grid.clone());
                        }
                        grid = rotate_grid(&grid);
                    }
                }
                let this_tile = this_tile.unwrap();
                let this_grid = this_grid.unwrap();

                add_to_main_grid(&mut filled_map, &offset, &this_grid);
                // print_grid(&filled_map);

                last_right = get_right(&this_grid);
                last_bottom = get_bottom(&this_grid);
                used_tiles.insert(this_tile.id);
            // let i = 0;
            } else {
                let mut this_tile = None;
                let mut this_grid = None;
                for tile in &tiles {
                    if used_tiles.contains(&tile.id) {
                        // println!("Skipping {}", tile.id);
                        continue;
                    }
                    let mut grid = tile.grid_data.map.clone();
                    for i in 0..4 {
                        let left = get_left(&grid);
                        if left == last_right {
                            // println!("(l) found matching left edge: {} {}", tile.id, left);
                            this_tile = Some(tile.clone());
                            this_grid = Some(grid.clone())
                        }
                        grid = rotate_grid(&grid);
                    }
                    let mut grid = tile.grid_data.map.clone();
                    grid = flip_grid(&grid, true, false);
                    for i in 0..4 {
                        let left = get_left(&grid);
                        if left == last_right {
                            // println!("(f) found matching left edge: {} {}", tile.id, left);
                            this_tile = Some(tile.clone());
                            this_grid = Some(grid.clone());
                        }
                        grid = rotate_grid(&grid);
                    }
                }
                let this_tile = this_tile.unwrap();
                let this_grid = this_grid.unwrap();

                add_to_main_grid(&mut filled_map, &offset, &this_grid);
                // print_grid(&filled_map);

                last_right = get_right(&this_grid);
                used_tiles.insert(this_tile.id);
                let i = 0;
            }
        }
    }
    // let i = 0;
    // print_grid(&filled_map, 10);
    let len = filled_map.iter().map(|e| e.0.x).max().unwrap();

    for y in 0..=len {
        for x in 0..=len {
            if x % 10 == 0 {
                let offset_x = x % 10 * 10;
                let col1 = Point::new(offset_x + x, y);
                // println!("clear {:?}", col1);

                let col2 = Point::new(offset_x + x + 9, y);
                // println!("clear {:?}", col2);

                filled_map.insert(col1, '*');
                filled_map.insert(col2, '*');
            }
            if y % 10 == 0 {
                let offset_y = y % 10 * 10;
                let row1 = Point::new(x, offset_y + y);
                // println!("clear {:?}", row1);

                let row2 = Point::new(x, offset_y + y + 9);
                // println!("clear {:?}", row2);

                filled_map.insert(row1, '*');
                filled_map.insert(row2, '*');
            }
        }
    }

    // print_grid(&filled_map, 10);

    let mut grid_edges_removed = HashMap::new();
    add_to_grid_with_filter(&mut grid_edges_removed, &filled_map, '*');

    // print_grid(&grid_edges_removed, 1000);

    let mut all_found_seamonsters = Vec::new();


    let mut grid = grid_edges_removed.clone();
    for _ in 0..4 {
        let found_seamonsters = find_seamonsters(&grid);
        all_found_seamonsters.push(found_seamonsters);
        grid = rotate_grid(&grid);
    }
    let mut grid = grid_edges_removed.clone();
    grid = flip_grid(&grid, true, false);
    for _ in 0..4 {
        let found_seamonsters = find_seamonsters(&grid);
        all_found_seamonsters.push(found_seamonsters);
        grid = rotate_grid(&grid);
    }

    let max_found = all_found_seamonsters.iter().max().unwrap();

    let all_hashes = grid_edges_removed.iter().filter(|e| *e.1 == '#').count();
    let rough_waters = all_hashes as i32 - (max_found * 15);
    println!("Part 2: {}", rough_waters);
}

fn find_seamonsters(grid: &HashMap<Point, char>) -> i32 {
    let raw_seamonster = "\
..................#.
#....##....##....###
.#..#..#..#..#..#..."
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();
    let seamonster_grid = GridData::parse_input(raw_seamonster);
    let seamoster_points = seamonster_grid
        .map
        .iter()
        .filter(|e| *e.1 == '#')
        .map(|e| e.0.clone())
        .collect::<Vec<Point>>();

    let len = grid.iter().map(|e| e.0.x).max().unwrap();

    let mut found_seamonsters = 0;
    for y in 0..=len {
        for x in 0..=len {
            let origin = Point::new(x, y);
            let mut matches = Vec::new();
            for sp in &seamoster_points {
                let test_point = Point::new(origin.x + sp.x, origin.y + sp.y);
                if let Some(val) = grid.get(&test_point) {
                    if val == &'#' {
                        matches.push(true);
                    }
                }
            }
            if matches.iter().all(|v| v == &true) && matches.len() == seamoster_points.len() {
                found_seamonsters += 1;
            }
        }
    }

    found_seamonsters
}

fn add_to_main_grid(
    filled_map: &mut HashMap<Point, char>,
    offset: &Point,
    grid: &HashMap<Point, char>,
) {
    let len = grid.iter().map(|e| e.0.x).max().unwrap();
    for y in 0..=len {
        for x in 0..=len {
            filled_map.insert(
                Point::new(offset.x + x, offset.y + y),
                *grid.get(&Point::new(x, y)).unwrap(),
            );
        }
    }
}

fn add_to_grid_with_filter(
    target_grid: &mut HashMap<Point, char>,
    grid: &HashMap<Point, char>,
    skip_char: char,
) {
    let len = grid.iter().map(|e| e.0.x).max().unwrap();
    let mut new_y = 0;
    let mut new_x = 0;
    let mut inserted_row = false;
    for y in 0..=len {
        new_x = 0;
        inserted_row = false;
        for x in 0..=len {
            let src_char = *grid.get(&Point::new(x, y)).unwrap();
            if src_char == skip_char {
                continue;
            }

            target_grid.insert(Point::new(new_x, new_y), src_char);
            inserted_row = true;
            new_x += 1;
        }
        if inserted_row {
            new_y += 1;
        }
    }
}

fn print_grid(grid: &HashMap<Point, char>, tile_size: i32) {
    let max_x = grid.iter().map(|e| e.0.x).max().unwrap();
    let max_y = grid.iter().map(|e| e.0.y).max().unwrap();

    for y in 0..=max_y {
        if y % tile_size == 0 {
            println!();
        }
        for x in 0..=max_x {
            if x % tile_size == 0 {
                print!(" ");
            }
            let found_entry = grid.get(&Point {
                x: x as i32,
                y: y as i32,
            });

            let print_c = if let Some(c) = found_entry { *c } else { ' ' };

            print!("{}", print_c)
        }

        println!();
    }
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
            let new_y = if flip_x { len - y } else { y };
            let new_x = if flip_y { len - x } else { x };

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


fn which_side_is_this(grid: &HashMap<Point, char>, side: &str) -> Side {
    if get_top(grid) == side {
        Top
    } else if get_bottom(grid) == side {
        Bottom
    } else if get_left(grid) == side {
        Left
    } else if get_right(grid) == side {
        Right
    } else {
        unreachable!()
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
        find_seamonsters, flip_grid, get_bottom, get_left, get_right, get_top,
        print_grid, rotate_grid,
    };
    use crate::util::{GridData, Point};
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
        print_grid(&grid, 10);

        let mut rotated_grid = grid.clone();
        for r in 1..4 {
            println!("rotate {}", r);
            rotated_grid = rotate_grid(&rotated_grid);
            print_grid(&rotated_grid, 10);
        }

        println!();

        print_grid(&grid, 10);
        println!("no flip");
        print_grid(&flip_grid(&grid, false, false), 10);
        println!("x flip");
        print_grid(&flip_grid(&grid, true, false), 10);
        println!("y flip");
        print_grid(&flip_grid(&grid, false, true), 10);
        println!("x y flip");
        print_grid(&flip_grid(&grid, true, true), 10);

        println!();
        println!("top: {}", get_top(&grid));
        println!("bottom: {}", get_bottom(&grid));
        println!("left: {}", get_left(&grid));
        println!("right: {}", get_right(&grid));
    }

    #[test]
    fn test_find_seamonsters() {
        let lines = "\
..................#.
#....##....##....###
.#..#..#..#..#..#...
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................
####################
####################
####################
....................
...................."
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        let gd = GridData::parse_input(lines);
        assert_eq!(2, find_seamonsters(&gd.map));
    }
}
