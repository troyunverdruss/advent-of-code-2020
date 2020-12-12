use crate::util::inputs::day_input;
use crate::util::{GridData, Point};
use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn run() {
    let input_lines = day_input(11);
    let grid_data = GridData::parse_input(input_lines);
    let part1 = part_1(&grid_data);
    println!("Part 1: {}", part1);
    let part2 = part_2(&grid_data);
    println!("Part 2: {}", part2);
}

fn part_1(grid_data: &GridData) -> usize {
    let mut a = HashMap::new();
    let mut b = HashMap::new();
    let mut memo = HashMap::new();
    let mut target_a = true;

    a = grid_data.map.clone();
    b = grid_data.map.clone();
    // let mut last_arrangement = grid_data.map.iter().sorted().map(|e| e.1).join("-");
    let mut loop_count = 0;
    let mut changes = 0;
    loop {
        changes = 0;
        let (mut src, mut dest) = if target_a {
            (&mut b, &mut a)
        } else {
            (&mut a, &mut b)
        };
        // println!("Loop: {}", loop_count);
        // io::stdout().flush();
        // GridData::debug_print(&GridData { map: src.clone(), rows: grid_data.rows, cols: grid_data.cols });
        // println!();
        // io::stdout().flush();
        for key in src.keys().sorted() {
            let neighbors = adjacent_neighbors(&key, &mut memo)
                .iter()
                .map(|p| src.get(p))
                .filter(|o| o.is_some())
                .map(|s| s.unwrap().to_owned())
                .collect::<Vec<String>>();

            let empty_count = neighbors
                .iter()
                .filter(|n| n.chars().next().unwrap() == '.' || n.chars().next().unwrap() == 'L')
                .count();
            let occupied_count = neighbors
                .iter()
                .filter(|n| n.chars().next().unwrap() == '#')
                .count();

            let current_seat = src.get(key).unwrap().chars().next().unwrap();

            match current_seat {
                'L' => {
                    if occupied_count == 0 {
                        dest.insert(key.clone(), '#'.to_string());
                        changes += 1;
                    } else {
                        dest.insert(key.clone(), current_seat.to_string());
                    }
                }
                '#' => {
                    if occupied_count >= 4 {
                        dest.insert(key.clone(), 'L'.to_string());
                        changes += 1;
                    } else {
                        dest.insert(key.clone(), current_seat.to_string());
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
            return dest.iter().map(|e| e.1).filter(|v| v == &"#").count();
        }
        loop_count += 1;
        // last_arrangement = current_arrangement;

        // if loop_count == 3 {
        //     break;
        // }
    }
    0
}

fn part_2(grid_data: &GridData) -> usize {
    let mut a = HashMap::new();
    let mut b = HashMap::new();
    let mut memo = HashMap::new();
    let mut target_a = true;

    a = grid_data.map.clone();
    b = grid_data.map.clone();
    // let mut last_arrangement = grid_data.map.iter().sorted().map(|e| e.1).join("-");
    let mut loop_count = 0;
    let mut changes = 0;
    loop {
        changes = 0;
        let (mut src, mut dest) = if target_a {
            (&mut b, &mut a)
        } else {
            (&mut a, &mut b)
        };
        *dest = src.clone();
        // println!("Loop: {}", loop_count);
        // GridData::debug_print(&GridData {
        //     map: src.clone(),
        //     rows: grid_data.rows,
        //     cols: grid_data.cols,
        // });
        // println!("src entries: {}", src.len());
        // println!();

        for key in src.keys().sorted() {
            let neighbors = visible_neighbors(&key, &grid_data.map, &mut memo)
                .iter()
                .map(|p| src.get(p))
                .filter(|o| o.is_some())
                .map(|s| s.unwrap().to_owned())
                .collect::<Vec<String>>();

            let occupied_count = neighbors
                .iter()
                .filter(|n| n.chars().next().unwrap() == '#')
                .count();

            let current_seat = src.get(key).unwrap().chars().next().unwrap();

            match current_seat {
                'L' => {
                    if occupied_count == 0 {
                        dest.insert(key.clone(), '#'.to_string());
                        changes += 1;
                    }
                }
                '#' => {
                    if occupied_count >= 5 {
                        dest.insert(key.clone(), 'L'.to_string());
                        changes += 1;
                    }
                }
                '.' => {}
                _ => {
                    panic!("Uh oh");
                }
            }
        }

        // GridData::debug_print(&GridData {
        //     map: dest.clone(),
        //     rows: grid_data.rows,
        //     cols: grid_data.cols,
        // });
        // println!("dest entries: {}", dest.len());
        // println!();

        target_a = !target_a;
        // let current_arrangement = dest.iter().sorted().map(|e| e.1).join("-");
        // if current_arrangement == last_arrangement {
        if changes == 0 {
            return dest.iter().map(|e| e.1).filter(|v| v == &"#").count();
        }
        loop_count += 1;
        // last_arrangement = current_arrangement;

        // if loop_count == 3 {
        //     break;
        // }
    }
    0
}

fn adjacent_neighbors(point: &Point, memo: &mut HashMap<Point, Vec<Point>>) -> Vec<Point> {
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
    map: &HashMap<Point, String>,
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
                    if entry.chars().next().unwrap() == 'L' || entry.chars().next().unwrap() == '#'
                    {
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
    use crate::day11::{part_1, part_2};
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

        let count = part_1(&grid_data);
        assert_eq!(37, count);
    }

    #[test]
    fn test_part_2() {
        let lines = LINES.iter().map((|s| String::from(*s))).collect();
        let grid_data = GridData::parse_input(lines);

        let count = part_2(&grid_data);
        assert_eq!(26, count);
    }
}
