use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

pub fn run() {
    let input = "167248359";
    let result = part1(input);
    println!("Part 1: {}" , result);
}

fn part1(input: &str) -> String {
    let all_cups: Vec<usize> = Vec::from_iter(
        input
            .chars()
            .map(|c| String::from(c).parse::<usize>().unwrap()),
    );
    let mut next_cup_order = all_cups
        .iter()
        .sorted()
        .rev()
        .copied()
        .collect::<VecDeque<usize>>();

    let mut cups = VecDeque::from_iter(all_cups);

    let mut step = 0;
    for step in 0..100 {
        println!("Step {}: cups: {:?}", step, cups);
        let current_cup = cups.front().unwrap().clone();
        cups.rotate_left(1);
        let a = cups.pop_front().unwrap();
        let b = cups.pop_front().unwrap();
        let c = cups.pop_front().unwrap();

        loop {
            if next_cup_order.back().unwrap() == cups.back().unwrap() {
                break;
            }
            next_cup_order.rotate_left(1);
        }
        loop {
            if !vec![a, b, c].contains(next_cup_order.front().unwrap()) {
                break;
            }
            next_cup_order.rotate_left(1);
        }

        loop {
            if cups.back().unwrap() == next_cup_order.front().unwrap() {
                break;
            }
            cups.rotate_left(1);
        }
        cups.push_front(c);
        cups.push_front(b);
        cups.push_front(a);

        loop {
            if *cups.back().unwrap() == current_cup {
                break
            }
            cups.rotate_left(1);
        }
    }

    loop {
        if *cups.back().unwrap() == 1 {
            break
        }
        cups.rotate_left(1);
    }

    let mut result = vec![];
    loop  {
        if *cups.front().unwrap() == 1 {
            break
        }
        result.push(*cups.front().unwrap());
        cups.rotate_left(1);
    }

    result.iter().map(|v| format!("{}", v)).join("")
}

#[cfg(test)]
mod tests {
    use crate::day23::part1;
    #[test]
    fn example1() {
        assert_eq!("67384529".to_owned(), part1(&"389125467"));
    }
}
