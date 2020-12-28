use crate::util::inputs::day_input;
use itertools::Itertools;

// TODO chinese remainder theorem? yes but how
pub fn run() {
    let lines = day_input(13);
    let min_bus = part1(&lines);

    println!(
        "Part 1: bus: {}, wait time: {}, mul: {}",
        min_bus.0,
        min_bus.1,
        min_bus.0 * min_bus.1
    );

    let timestamp = part2(&lines);
    println!("Part 2: {}", timestamp);
}

fn part1(lines: &[String]) -> (usize, usize) {
    let departure_time = lines.get(0).unwrap().parse::<usize>().unwrap();
    let buses = lines
        .get(1)
        .unwrap()
        .split(',')
        .filter(|v| v != &"x")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let min_bus = buses
        .iter()
        .map(|bus| {
            let remainder = departure_time % bus;
            (bus, bus - remainder)
        })
        .min_by(|a, b| a.to_owned().1.cmp(&b.to_owned().1))
        .unwrap();

    (min_bus.0.to_owned(), min_bus.1)
}

fn part2(lines: &[String]) -> i64 {
    let buses_with_index = lines
        .get(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|t| t.1 != "x")
        .map(|t| (t.0, t.1.parse::<usize>().unwrap()))
        .collect::<Vec<(usize, usize)>>();

    // println!("{:?}", buses_with_index);

    let buses: Vec<usize> = buses_with_index.iter().map(|t| t.1).collect();
    let remainders: Vec<usize> = buses_with_index.iter().map(|t| t.0).collect();

    let mut sorted_pairs = buses
        .iter()
        .zip(remainders)
        .sorted_by(|a, b| a.0.cmp(b.0))
        .map(|e| (*e.0, e.1))
        .collect::<Vec<(usize, usize)>>();

    let mut t = 0;
    let mut step_size = 1;
    let (mut bus, mut rem) = sorted_pairs.pop().unwrap();
    loop {
        if (t + rem) % bus == 0 {
            // println!("{} % {} = {}", t, bus, rem);
            if sorted_pairs.is_empty() {
                // println!("Found solution: {}", t);
                break;
            }
            // println!("Old step size {}", step_size);
            step_size *= bus;
            // println!("New step size {}", step_size);

            let pair = sorted_pairs.pop().unwrap();
            bus = pair.0;
            rem = pair.1;
            // println!("Next search pair: {:?}", pair);
        }

        t += step_size
    }

    t as i64
}

#[cfg(test)]
mod tests {
    use crate::day13::{part1, part2};

    #[test]
    fn test_part1() {
        let lines = vec!["939".to_owned(), "7,13,x,x,59,x,31,19".to_owned()];

        let result = part1(&lines);
        assert_eq!(295, result.0 * result.1);
    }

    #[test]
    fn test_part2_1() {
        let lines = vec!["".to_owned(), "7,13,x,x,59,x,31,19".to_owned()];

        let result = part2(&lines);
        assert_eq!(1068781, result);
    }

    #[test]
    fn test_part2_2() {
        let lines = vec!["".to_owned(), "17,x,13,19".to_owned()];

        let result = part2(&lines);
        assert_eq!(3417, result);
    }
}
