use crate::day24::Direction::{E, NE, NW, SE, SW, W};
use crate::util::inputs::day_input;
use crate::util::Point;
use itertools::__std_iter::FromIterator;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let lines = day_input(24);
    let part1_result = part1(&lines);
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&lines, 100);
    println!("Part 2: {}", part2_result);
}

fn part1(lines: &[String]) -> usize {
    let grid = setup_initial_state(lines);

    grid.iter().filter(|e| e.1 == &true).count()
}

fn setup_initial_state(lines: &[String]) -> HashMap<Point, bool> {
    let ends = lines
        .iter()
        .map(parse_instruction)
        .map(|d| follow_to_end(&d))
        .collect::<Vec<Point>>();

    let mut results: HashMap<Point, bool> = HashMap::new();
    for end in &ends {
        let entry = results.get(end);
        if let Some(val) = entry {
            results.insert(end.clone(), !*val);
        } else {
            results.insert(end.clone(), true);
        }
    }
    results
}

fn part2(lines: &[String], iterations: usize) -> usize {
    let mut grid = setup_initial_state(lines);
    for _ in 0..iterations {
        let black_tiles = grid
            .iter()
            .filter(|e| e.1 == &true)
            .map(|e| e.0.clone())
            .collect::<HashSet<Point>>();
        let possible_white_tiles = black_tiles
            .iter()
            .map(|p| neighbors(p))
            .flatten()
            .collect::<HashSet<Point>>();

        let mut all_tiles = HashSet::new();
        all_tiles.extend(black_tiles);
        all_tiles.extend(possible_white_tiles);

        let mut changes: Vec<(Point, bool)> = Vec::new();
        changes.reserve(all_tiles.len());

        for point in &all_tiles {
            let is_black = if let Some(val) = grid.get(point) {
                *val
            } else {
                false
            };

            let mut black_neighbors = 0;
            for neighbor in neighbors(point) {
                let entry = grid.entry(neighbor).or_insert(false);
                if *entry {
                    black_neighbors += 1;
                }
            }

            if is_black {
                if black_neighbors == 0 || black_neighbors > 2 {
                    changes.push((point.clone(), false))
                }
            } else if black_neighbors == 2 {
                changes.push((point.clone(), true))
            }
        }

        for change in changes {
            grid.insert(change.0, change.1);
        }
    }

    grid.iter().filter(|e| e.1 == &true).count()
}

fn neighbors(loc: &Point) -> Vec<Point> {
    vec![
        loc.clone() + NE.delta(),
        loc.clone() + SE.delta(),
        loc.clone() + E.delta(),
        loc.clone() + NW.delta(),
        loc.clone() + SW.delta(),
        loc.clone() + W.delta(),
    ]
}

fn parse_instruction(line: &String) -> Vec<Direction> {
    let instructions = line.replace("e", "e_").replace("w", "w_");
    let splits = instructions.split('_');
    splits
        .into_iter()
        .filter(|v| !v.is_empty())
        .map(|v| Direction::parse(v))
        .collect::<Vec<Direction>>()
}

fn follow_to_end(directions: &[Direction]) -> Point {
    let origin = Point::new(0, 0);
    directions
        .iter()
        .map(|d| d.delta())
        .fold(origin, |acc, d| acc + d)
}

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    NE,
    SE,
    E,
    NW,
    SW,
    W,
}

impl Direction {
    fn delta(&self) -> Point {
        match self {
            NE => Point::new(1, 1),
            SE => Point::new(1, -1),
            E => Point::new(2, 0),
            NW => Point::new(-1, 1),
            SW => Point::new(-1, -1),
            W => Point::new(-2, 0),
        }
    }
}

impl Direction {
    fn parse(s: &str) -> Direction {
        match s {
            "ne" => NE,
            "se" => SE,
            "e" => E,
            "nw" => NW,
            "sw" => SW,
            "w" => W,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day24::Direction::{E, NE, SE, W};
    use crate::day24::{follow_to_end, parse_instruction, part1, part2};
    use crate::util::Point;

    #[test]
    fn test_parse_instructions() {
        assert_eq!(vec![E, SE, W], parse_instruction(&String::from("esew")));
        assert_eq!(vec![E, NE, E], parse_instruction(&String::from("enee")));
    }

    #[test]
    fn part_1_example() {
        let lines = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(10, part1(&lines));
    }

    #[test]
    fn test_follow_directions() {
        assert_eq!(
            Point::new(1, -1),
            follow_to_end(&parse_instruction(&String::from("esew")))
        );

        assert_eq!(
            Point::new(0, 0),
            follow_to_end(&parse_instruction(&String::from("nwwswee")))
        );
    }
    #[test]
    fn test_example_part_2() {
        let lines = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(15, part2(&lines, 1));
        assert_eq!(37, part2(&lines, 10));
        assert_eq!(132, part2(&lines, 20));
        assert_eq!(2208, part2(&lines, 100));
    }
}
