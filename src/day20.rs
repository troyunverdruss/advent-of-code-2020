use crate::util::inputs::read_lines_split_by_double_newline;
use std::collections::HashSet;
use crate::util::GridData;

pub fn run () {
    let tiles = read_lines_split_by_double_newline(20);

}

// fn parse_tile(line: &String) -> Tile{
//
// }

struct Tile {
    id: i32,
    possible_short: HashSet<String>,
    possible_long: HashSet<String>,
    grid_data: GridData
}
