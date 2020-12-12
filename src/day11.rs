use crate::util::inputs::day_input;
use crate::util::{GridData, Point};
use itertools::Itertools;
use std::collections::HashMap;

// TODO why so slow? 
pub fn run() {
    let input_lines = day_input(11);
    let grid_data = GridData::parse_input(input_lines);

    let part1 = solver(&grid_data, 4, adjacent_neighbors);
    println!("Part 1: {}", part1);

    let part2 = solver(&grid_data, 5, visible_neighbors);
    println!("Part 2: {}", part2);
}

type NeighborProvider = fn(
    point: &Point,
    map: &HashMap<Point, char>,
    memo: &mut HashMap<Point, Vec<Point>>,
) -> Vec<Point>;

fn solver(
    grid_data: &GridData,
    required_occupied_seats: usize,
    neighbor_provider: NeighborProvider,
) -> usize {
    let mut a;
    let mut b;
    let mut memo = HashMap::new();
    let mut target_a = true;

    a = grid_data.map.clone();
    b = grid_data.map.clone();
    // let mut last_arrangement = grid_data.map.iter().sorted().map(|e| e.1).join("-");
    let mut changes;
    loop {
        changes = 0;
        let (src, dest) = if target_a {
            (&mut b, &mut a)
        } else {
            (&mut a, &mut b)
        };
        // dest = &mut src.clone();
        // println!("Loop: {}", loop_count);
        // io::stdout().flush();
        // GridData::debug_print(&GridData { map: src.clone(), rows: grid_data.rows, cols: grid_data.cols });
        // println!();
        // io::stdout().flush();
        for key in src.keys().sorted() {
            let neighbors = neighbor_provider(&key, &grid_data.map, &mut memo)
                .iter()
                .map(|p| src.get(p))
                .filter(|o| o.is_some())
                .map(|s| s.unwrap().to_owned())
                .collect::<Vec<char>>();

            let occupied_count = neighbors.iter().filter(|n| n==&&'#').count();

            let current_seat = src.get(key).unwrap();

            match current_seat {
                'L' => {
                    if occupied_count == 0 {
                        dest.insert(key.clone(), '#');
                        changes += 1;
                    } else {
                        dest.insert(key.clone(), current_seat.to_owned());
                    }
                }
                '#' => {
                    if occupied_count >= required_occupied_seats {
                        dest.insert(key.clone(), 'L');
                        changes += 1;
                    } else {
                        dest.insert(key.clone(), current_seat.to_owned());
                    }
                }
                '.' => {}
                _ => {
                    panic!("Uh oh");
                }
            }
        }

        // GridData::debug_print(&GridData { map: dest.clone(), rows: grid_data.rows, cols: grid_data.cols });
        // println!();

        target_a = !target_a;
        // let current_arrangement = dest.iter().sorted().map(|e| e.1).join("-");
        // if current_arrangement == last_arrangement {
        if changes == 0 {
            return dest.iter().map(|e| e.1).filter(|v| v == &&'#').count();
        }
    }
}

fn adjacent_neighbors(
    point: &Point,
    _map: &HashMap<Point, char>,
    memo: &mut HashMap<Point, Vec<Point>>,
) -> Vec<Point> {
    if let Some(val) = memo.get(point) {
        return val.to_owned();
    }

    let result = vec![
        Point {
            x: point.x - 1,
            y: point.y - 1,
        },
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point {
            x: point.x + 1,
            y: point.y - 1,
        },
        Point {
            x: point.x - 1,
            y: point.y,
        },
        // point
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: point.x - 1,
            y: point.y + 1,
        },
        Point {
            x: point.x,
            y: point.y + 1,
        },
        Point {
            x: point.x + 1,
            y: point.y + 1,
        },
    ];

    memo.insert(point.to_owned(), result);
    memo.get(point).unwrap().to_owned()
}

fn visible_neighbors(
    point: &Point,
    map: &HashMap<Point, char>,
    memo: &mut HashMap<Point, Vec<Point>>,
) -> Vec<Point> {
    if let Some(val) = memo.get(point) {
        // dbg!("memo hit");
        return val.to_owned();
    }
    // println!("memo miss");

    let mut results = vec![];

    let slopes = vec![
        Point { x: -1, y: -1 },
        Point { x: 0, y: -1 },
        Point { x: 1, y: -1 },
        Point { x: -1, y: 0 },
        // point
        Point { x: 1, y: 0 },
        Point { x: -1, y: 1 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: 1 },
    ];
    for slope in slopes {
        let mut count = 1;
        loop {
            let test_point = Point {
                x: point.x + slope.x * count,
                y: point.y + slope.y * count,
            };
            match map.get(&test_point) {
                None => break,
                Some(entry) => {
                    if entry == &'L' || entry == &'#' {
                        results.push(test_point);
                        break;
                    } else {
                        count += 1;
                    }
                }
            }
        }
    }

    memo.insert(point.to_owned(), results);
    memo.get(point).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::day11::{adjacent_neighbors, solver, visible_neighbors};
    use crate::util::GridData;

    static LINES: [&str; 10] = [
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];

    #[test]
    fn test_part_1() {
        let lines = LINES.iter().map(|s| String::from(*s)).collect();
        let grid_data = GridData::parse_input(lines);

        let count = solver(&grid_data, 4, adjacent_neighbors);
        assert_eq!(37, count);
    }

    #[test]
    fn test_part_2() {
        let lines = LINES.iter().map(|s| String::from(*s)).collect();
        let grid_data = GridData::parse_input(lines);

        let count = solver(&grid_data, 5, visible_neighbors);
        assert_eq!(26, count);
    }
}
