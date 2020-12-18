use crate::util::inputs::day_input;
use crate::util::{GridData, Point3, Point4};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let lines = day_input(17);
    let grid_data = GridData::parse_input(lines);
    let part1_result = part1(&grid_data, 3);
    println!("Part 1: {}", part1_result);

    let part2_result = part1(&grid_data, 4);
    println!("Part 2: {}", part2_result);
}

fn part1(grid_data: &GridData, dims: i32) -> usize {
    let mut grid = HashMap::new();

    // Convert the parsed points to 3d points
    grid_data.map.iter().for_each(|e| {
        grid.insert(Point4::from2d(e.0), *e.1);
    });

    for loop_id in 0..6 {
        println!(
            "Start loop {}, count: {}",
            loop_id,
            grid.values().filter(|v| v == &&'#').count()
        );
        let existing_keys = grid.keys().map(|k| k.to_owned()).collect::<Vec<Point4>>();
        let mut keys_for_this_cycle = HashSet::new();
        for k in existing_keys {
            keys_for_this_cycle.insert(k.clone());
            keys_for_this_cycle.extend(neighbors(&k, dims));
        }

        let mut updates = vec![];
        updates.reserve(keys_for_this_cycle.len());
        for point in keys_for_this_cycle {
            grid.entry(point.clone()).or_insert('.');
            let state = next_state(&grid, &point, dims);
            updates.push((point.clone(), state))
        }
        for u in updates {
            grid.insert(u.0, u.1);
        }
        println!(
            "End loop {}, count: {}",
            loop_id,
            grid.values().filter(|v| v == &&'#').count()
        );
    }

    grid.values().filter(|v| v == &&'#').count()
}

fn next_state(grid: &HashMap<Point4, char>, loc: &Point4, dims: i32) -> char {
    let neighbors = neighbors(loc, dims);
    let current = grid.get(loc).expect("should exist");
    match current {
        '#' => handle_active(&grid, &neighbors),
        '.' => handle_inactive(&grid, &neighbors),
        _ => unreachable!("wtf"),
    }
}

fn handle_active(grid: &HashMap<Point4, char>, neighbors: &HashSet<Point4>) -> char {
    // If a cube is active and exactly 2 or 3 of its
    // neighbors are also active, the cube remains active.
    // Otherwise, the cube becomes inactive.
    let mut active = 0;
    for n in neighbors {
        if let Some(val) = grid.get(&n) {
            if *val == '#' {
                active += 1;
            }
        }
    }
    if active == 2 || active == 3 {
        '#'
    } else {
        '.'
    }
}

fn handle_inactive(grid: &HashMap<Point4, char>, neighbors: &HashSet<Point4>) -> char {
    // If a cube is inactive but exactly 3 of its
    // neighbors are active, the cube becomes active.
    // Otherwise, the cube remains inactive.
    let mut active = 0;
    for n in neighbors {
        if let Some(val) = grid.get(&n) {
            if *val == '#' {
                active += 1;
            }
        }
    }
    if active == 3 {
        '#'
    } else {
        '.'
    }
}

fn neighbors(point: &Point4, dims: i32) -> HashSet<Point4> {
    let mut result = HashSet::new();
    for x in point.x - 1..=point.x + 1 {
        for y in point.y - 1..=point.y + 1 {
            for z in point.z - 1..=point.z + 1 {
                for w in point.w - 1..=point.w + 1 {
                    let n = if dims == 3 {
                        Point4 { x, y, z, w: 0 }
                    } else {
                        Point4 { x, y, z, w }
                    };

                    if n == *point {
                        continue;
                    }
                    result.insert(n);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day17::part1;
    use crate::util::GridData;

    #[test]
    fn example_1() {
        let lines = "\
.#.
..#
###"
        .split('\n')
        .map(String::from)
        .collect();

        let grid_data = GridData::parse_input(lines);
        assert_eq!(112, part1(&grid_data, 3));
    }
    #[test]
    fn example_2() {
        let lines = "\
.#.
..#
###"
        .split('\n')
        .map(String::from)
        .collect();

        let grid_data = GridData::parse_input(lines);
        assert_eq!(848, part1(&grid_data, 4));
    }
}
