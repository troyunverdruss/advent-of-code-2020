use crate::util::inputs::day_input;
use itertools::min;

pub fn run() {
    let lines = day_input(13);
    let min_bus = part1(&lines);

    println!(
        "Part 1: bus: {}, wait time: {}, mul: {}",
        min_bus.0,
        min_bus.1,
        min_bus.0 * min_bus.1
    )
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

    let i = 9;

    0
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
