use crate::util;
use crate::util::{GridData, Point};

pub fn run() {
    let lines = util::day_input(3);
    let input = GridData::parse_input(lines);

    let trees = run_through_trees(&input, &Slope { dx: 3, dy: 1 });
    println!("Part 1, tree collisions count: {}", trees);

    let slopes = vec![
        Slope { dx: 1, dy: 1 },
        Slope { dx: 3, dy: 1 },
        Slope { dx: 5, dy: 1 },
        Slope { dx: 7, dy: 1 },
        Slope { dx: 1, dy: 2 },
    ];

    let all_trees: i32 = slopes
        .iter()
        .map(|s| run_through_trees(&input, &s))
        .product();
    println!("Part 1, trees collisions product: {}", all_trees);
}

struct Slope {
    dx: usize,
    dy: usize,
}

fn run_through_trees(grid_data: &GridData, slope: &Slope) -> i32 {
    let mut trees = 0;
    let mut x = 0;

    // input.debug_print();

    for y in (0..grid_data.rows).step_by(slope.dy) {
        let coord = Point {
            x: x % grid_data.cols,
            y,
        };

        let val = grid_data.map.get(&coord).unwrap();
        if val == "#" {
            trees += 1;
        }

        x += slope.dx;
    }

    trees
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1_example() {}

    #[test]
    fn split_example() {
        let split_str: Vec<String> = "abc".split("").map(String::from).collect();
        let split_char: Vec<char> = "abc".chars().into_iter().collect();

        let i = 0;
    }
}
