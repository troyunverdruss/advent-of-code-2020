use crate::util::inputs::day_input;
use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let adapters = day_input(10)
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .sorted()
        .collect::<Vec<i32>>();

    let part1 = part_1(&adapters);
    println!("Part 1: {}", part1);

    let part2 = part_2(&adapters);
    println!("Part 2: {}", part2);
}

fn part_1(adapters: &[i32]) -> i32 {
    let mut results = HashMap::new();
    results.insert(1, 0);
    results.insert(2, 0);
    // 3 starts at 1 because we always have a 3 at the end to get to our device
    results.insert(3, 1);

    let mut last_val = 0;
    for a in adapters {
        let change = *a - last_val;
        match change {
            1 => results.entry(1).and_modify(|f| *f += 1),
            2 => results.entry(2).and_modify(|f| *f += 1),
            3 => results.entry(3).and_modify(|f| *f += 1),
            _ => {
                let error = format!("{} adapter greater than 3 from {}", a, last_val);
                panic!(error);
            }
        };
        last_val = *a;
    }
    let ones = results.get(&1).unwrap();
    let threes = results.get(&3).unwrap();

    ones * threes
}

// TODO Tribonnaci?
// Dynamic programming? 
// counting sort?
// https://www.reddit.com/r/adventofcode/comments/ka8z8x/2020_day_10_solutions/gfal951/?utm_source=share&utm_medium=ios_app&utm_name=iossmf&context=3
fn part_2(adapters: &Vec<i32>) -> i64 {
    let mut memo = HashMap::new();
    let result = solver(&0, &adapters[..], &mut memo).unwrap();
    println!("memo size: {}", memo.len());
    result
}

fn solver(
    current: &i32,
    remaining: &[i32],
    memo: &mut HashMap<String, Option<i64>>,
) -> Option<i64> {
    let key = remaining.iter().join("-");
    if let Some(key) = memo.get(&key) {
        if let Some(value) = key {
            return Some(*value);
        }
    }

    let result = if remaining.is_empty() {
        Some(1)
    } else {
        let next_steps: Vec<i32> = remaining
            .iter()
            .map(|v| *v)
            .filter(|v| *v - current <= 3)
            .collect();
        assert!(next_steps.len() <= 3);

        if next_steps.len() == 0 {
            None
        } else {
            let mut sum = 0;
            for (index, value) in next_steps.iter().enumerate() {
                let result = solver(value, &remaining[index + 1..], memo);
                if let Some(val) = result {
                    sum += val;
                }
            }
            if sum == 0 {
                None
            } else {
                Some(sum)
            }
        }
    };

    memo.insert(key.to_owned(), result.to_owned());
    result
}

#[cfg(test)]
mod tests {
    use crate::day10::{part_1, part_2};

    #[test]
    fn example_1_1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();
        assert_eq!(35, part_1(&adapters))
    }

    #[test]
    fn example_1_2() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        adapters.sort();
        assert_eq!(220, part_1(&adapters))
    }

    #[test]
    fn example_2_1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        adapters.sort();
        assert_eq!(8, part_2(&adapters))
    }

    #[test]
    fn example_2_2() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        adapters.sort();
        assert_eq!(19208, part_2(&adapters))
    }
}
