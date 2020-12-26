use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

pub fn run() {
    let input = "167248359";
    let result = part2(input, 9, 100);
    println!("Part 1: {}", result.part1);
    let result = part2(input, 1_000_000, 10_000_000);
    println!("Part 2: {}", result.part2);
}

fn part2(input: &str, highest_cup: usize, iterations: usize) -> GameResult {
    let mut cups : Vec<i64> = vec![0; highest_cup + 1];
    let input_cups: Vec<i64> = Vec::from_iter(
        input
            .chars()
            .map(|c| String::from(c).parse::<i64>().unwrap()),
    );

    let start = *input_cups.get(0).unwrap();

    let mut cup_to_set = start;

    for i in 1..input_cups.len() {
        let next = *input_cups.get(i).unwrap();
        cups[cup_to_set as usize] = next;
        cup_to_set = next as i64;
    }

    if highest_cup > 9 {
        for i in 10..=highest_cup {
            cups[cup_to_set as usize] = i as i64;
            cup_to_set = i as i64;
        }
    }

    cups[cup_to_set as usize] = start;

    // ok cups are "set up", time to start shuffling these things

    let mut start_n = start;
    for _ in 0..iterations {
        let a = cups[start_n as usize];
        let b = cups[a as usize];
        let c = cups[b as usize];
        let next_start_d = cups[c as usize];

        cups[start_n as usize] = next_start_d;
        let insertion_point_e = {
            let mut next = start_n;
            loop {
                if next == 1 {
                    next = highest_cup as i64
                } else {
                    next -= 1
                };

                if next != a && next != b && next != c {
                    break;
                }
            }
            next
        };
        let next_cup_after_insertion_f = cups[insertion_point_e as usize];
        cups[insertion_point_e as usize] = a;
        cups[c as usize ] = next_cup_after_insertion_f;
        start_n = next_start_d;
    }

    // Get part 1 result
    let mut part1_result = vec![];
    let mut next = 1;
    for _ in 0..8 {
        next = cups[next as usize];
        part1_result.push(next);
    }
    let part1_result = part1_result.iter().map(|v| format!("{}", v)).join("");

    // Get part 2 result
    let part2_a = cups[1];
    let part2_b = cups[part2_a as usize];
    let part2_result = part2_a * part2_b;

    GameResult {
        part1: part1_result,
        part2: part2_result
    }
}

struct GameResult {
    part1: String,
    part2: i64,
}

#[cfg(test)]
mod tests {
    use crate::day23::part2;
    #[test]
    fn part1_example() {
        assert_eq!("67384529".to_owned(), part2(&"389125467", 9, 100).part1);
    }

    #[test]
    fn part2_example() {
        assert_eq!(149245887792, part2(&"389125467", 1_000_000, 10_000_000).part2);
    }
}
