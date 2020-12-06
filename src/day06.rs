use crate::util;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let big_line = util::read_lines_to_string(6);
    let groups = big_line.split("\n\n").collect::<Vec<&str>>();

    let part1: usize = groups
        .iter()
        .map(|g| {
            g.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();
    println!("Part 1: {}", part1);

    let total = part_2(&groups);
    println!("Part 2: {}", total);

    let total = part_2_functional(&groups);
    println!("Part 2 (functional): {}", total);
}

fn part_2(groups: &Vec<&str>) -> usize {
    let mut total = 0;
    for group in groups {
        let mut answers: HashMap<char, usize> = HashMap::new();
        let people = group
            .split("\n")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let group_size = people.len();
        for person in people {
            person.chars().filter(|c| !c.is_whitespace()).for_each(|c| {
                if answers.get(&c).is_none() {
                    answers.insert(c, 1);
                } else {
                    answers.insert(c, answers.get(&c).unwrap() + 1);
                }
            });
        }
        total += answers.iter().filter(|e| e.1 == &group_size).count();
    }
    total
}

fn part_2_functional(groups: &Vec<&str>) -> usize {
    groups
        .iter()
        .map(|group| {
            group
                .split("\n")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .map(|people| {
            people
                .iter()
                .map(|p| {
                    p.chars()
                        .filter(|c| !c.is_whitespace())
                        .map(|c| (c, 1))
                        .collect::<Vec<(char, usize)>>()
                })
                .flatten()
                .into_group_map()
                .iter()
                .filter(|e| e.1.len() == people.len())
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day06::{part_2, part_2_functional};
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn part_2_test() {
        let groups = vec!["abc\n", "a\nb\nc\n", "ab\nac", "a\na\na\na\n", "b\n"];
        assert_eq!(6, part_2(&groups));
    }

    #[test]
    fn part_2_test_functional() {
        let groups = vec!["abc\n", "a\nb\nc\n", "ab\nac", "a\na\na\na\n", "b\n"];
        assert_eq!(6, part_2_functional(&groups));
    }
}
