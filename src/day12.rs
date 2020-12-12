use crate::util::inputs::day_input;
use crate::util::Point;

static NORTH: Point = Point { x: 0, y: 1 };
static SOUTH: Point = Point { x: 0, y: -1 };
static EAST: Point = Point { x: 1, y: 0 };
static WEST: Point = Point { x: -1, y: 0 };

// TODO complex numbers? 
pub fn run() {
    let lines = day_input(12);
    let instructions = parse_to_instructions(lines);

    let distance = part1(&instructions);
    println!("Part 1: {}", distance);

    let distance = part2(&instructions);
    println!("Part 2: {}", distance);
}

fn parse_to_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .map(|l| {
            let dir = l.chars().next().unwrap();
            let amt = l[1..].parse::<i32>().unwrap();

            Instruction { dir, amt }
        })
        .collect::<Vec<Instruction>>()
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut position = Point { x: 0, y: 0 };
    let mut direction = 'E';

    let moves = vec!['N', 'S', 'E', 'W'];
    let turns = vec!['L', 'R'];
    for inst in instructions {
        // println!("inst: {:?}", inst);
        if moves.contains(&inst.dir) {
            let slope = get_slope(&inst.dir);
            position = Point {
                x: position.x + slope.x * inst.amt,
                y: position.y + slope.y * inst.amt,
            };
        } else if turns.contains(&inst.dir) {
            direction = change_direction(&direction, &inst.dir, &inst.amt);
        } else if inst.dir == 'F' {
            let slope = get_slope(&direction);
            position = Point {
                x: position.x + slope.x * inst.amt,
                y: position.y + slope.y * inst.amt,
            };
        } else {
            panic!("wtf");
        }
    }

    manhattan_distance(&Point { x: 0, y: 0 }, &position)
}

fn part2(instructions: &[Instruction]) -> i32 {
    let mut boat_position = Point { x: 0, y: 0 };
    let mut waypoint = Point { x: 10, y: 1 };

    let moves = vec!['N', 'S', 'E', 'W'];
    let turns = vec!['L', 'R'];

    // println!("Boat:     {:?}", boat_position);
    // println!("Waypoint: {:?}", waypoint);
    // println!();
    for inst in instructions {
        // println!("inst: {:?}", inst);
        // println!("Boat:     {:?}", boat_position);
        // println!("Waypoint: {:?}", waypoint);
        // println!();
        if moves.contains(&inst.dir) {
            let slope = get_slope(&inst.dir);
            waypoint = Point {
                x: waypoint.x + slope.x * inst.amt,
                y: waypoint.y + slope.y * inst.amt,
            };
        } else if turns.contains(&inst.dir) {
            waypoint = move_waypoint(&waypoint, &inst.dir, &inst.amt);
        } else if inst.dir == 'F' {
            boat_position = Point {
                x: boat_position.x + waypoint.x * inst.amt,
                y: boat_position.y + waypoint.y * inst.amt,
            };
        } else {
            panic!("wtf");
        }
        // println!("Result:");
        // println!("Boat:     {:?}", boat_position);
        // println!("Waypoint: {:?}", waypoint);
        // println!();
    }

    manhattan_distance(&Point { x: 0, y: 0 }, &boat_position)
}

fn manhattan_distance(start: &Point, end: &Point) -> i32 {
    (start.x - end.x).abs() + (start.y - end.y).abs()
}

fn change_direction(current_dir: &char, rotation_dir: &char, amt: &i32) -> char {
    let ordered_right = vec!['N', 'E', 'S', 'W'];
    let ordered_left = vec!['N', 'W', 'S', 'E'];

    let turn_places = amt / 90;
    let curr_index = match rotation_dir {
        'L' => ordered_left
            .iter()
            .position(|m| m == current_dir)
            .expect("unknown dir") as i32,
        'R' => ordered_right
            .iter()
            .position(|m| m == current_dir)
            .expect("unknown dir") as i32,
        _ => panic!("unknown rotation dir"),
    };

    match rotation_dir {
        'L' => ordered_left
            .get(((curr_index + turn_places) % 4) as usize)
            .unwrap()
            .to_owned(),
        'R' => ordered_right
            .get(((curr_index + turn_places) % 4) as usize)
            .unwrap()
            .to_owned(),
        _ => panic!("unknown rotation dir"),
    }
}

fn move_waypoint(waypoint: &Point, direction: &char, amt: &i32) -> Point {
    let dx = waypoint.x;
    let dy = waypoint.y;

    let delta = match direction {
        'R' => match amt {
            90 => Point { x: dy, y: -dx },
            180 => Point { x: -dx, y: -dy },
            270 => Point { x: -dy, y: dx },
            _ => panic!("right fail"),
        },
        'L' => match amt {
            90 => Point { x: -dy, y: dx },
            180 => Point { x: -dx, y: -dy },
            270 => Point { x: dy, y: -dx },
            _ => panic!("left fail"),
        },
        _ => panic!("fail fail"),
    };

    Point {
        x: delta.x,
        y: delta.y,
    }
}

fn get_slope(current_dir: &char) -> Point {
    match current_dir {
        'N' => NORTH.clone(),
        'S' => SOUTH.clone(),
        'E' => EAST.clone(),
        'W' => WEST.clone(),
        _ => panic!("What direction?"),
    }
}

#[derive(Debug)]
struct Instruction {
    dir: char,
    amt: i32,
}

#[cfg(test)]
mod tests {
    use crate::day12::{change_direction, parse_to_instructions, part2};

    #[test]
    fn test_change_dir() {
        assert_eq!('W', change_direction(&'N', &'L', &90));
        assert_eq!('E', change_direction(&'W', &'L', &180));
        assert_eq!('S', change_direction(&'E', &'L', &270));

        assert_eq!('E', change_direction(&'N', &'R', &90));
        assert_eq!('N', change_direction(&'S', &'R', &180));
        assert_eq!('N', change_direction(&'E', &'R', &270));
    }

    #[test]
    fn test_part2() {
        let instructions = vec![
            "F10".to_owned(),
            "N3".to_owned(),
            "F7".to_owned(),
            "R90".to_owned(),
            "F11".to_owned(),
        ];
        assert_eq!(286, part2(&parse_to_instructions(instructions)));
    }
}
