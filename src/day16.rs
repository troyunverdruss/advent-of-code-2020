use crate::util::inputs::{day_input, read_lines_split_by_double_newline};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn run() {
    let groups = read_lines_split_by_double_newline(16);

    let (rules_str, my_ticket, nearby_tickets) = parse_input(&groups);

    let valid_numbers = parse_rules_to_set(&rules_str);
    let ticket_scanning_error = compute_scanning_error_rate(&valid_numbers, &nearby_tickets);
    println!(
        "Part 1: ticket scanning error rate: {}",
        ticket_scanning_error
    );

    let (tickets_as_nums, results) = part_2(&rules_str, &my_ticket, &nearby_tickets, &valid_numbers);

    let my_ticket_nums = tickets_as_nums.get(0).unwrap();

    let result = compute_final_result_part_2(&results, &my_ticket_nums, "departure");
    println!("Part 2: final result: {}", result);
    // too low 137984

    let i = 0;
}

fn compute_final_result_part_2(results: &HashMap<String, usize>, my_ticket_nums: &Vec<i64>, pattern: &str) -> i64 {
    let mut result = 1;
    for key in results.keys() {
        if key.starts_with(pattern) {
            result *= my_ticket_nums.get(*results.get(key).unwrap()).unwrap();
        }
    }
    result
}

fn part_2(rules_str: &Vec<&str>, my_ticket: &String, nearby_tickets: &Vec<&str>, valid_numbers: &HashSet<i64>) -> (Vec<Vec<i64>>, HashMap<String, usize, RandomState>) {
    let mut all_tickets = vec![&my_ticket[..]];
    all_tickets.extend(nearby_tickets.iter());

    let all_valid_tickets: Vec<&str> = all_tickets
        .iter()
        .filter(|l| !l.starts_with("nearby"))
        .filter(|l| compute_scanning_error_rate(&valid_numbers, &[l]) == 0)
        .copied()
        .collect();

    let tickets_as_nums: Vec<Vec<i64>> = all_valid_tickets
        .iter()
        .map(|tl| {
            tl.split(',')
                .map(|v| String::from(v).parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let rules: Vec<Rule> = rules_str.iter().map(|s| parse_rules_to_struct(s)).collect();
    let grouped_tickets: HashMap<usize, Vec<i64>> = tickets_as_nums
        .iter()
        .map(|t| {
            let x = t
                .iter()
                .enumerate()
                .map(|e| (e.0, *e.1))
                .collect::<Vec<(usize, i64)>>();
            x
        })
        .flatten()
        .into_group_map();

    let mut candidates = HashMap::new();
    for rule in rules {
        let entry = candidates.entry(rule.name).or_insert(HashSet::new());

        for group_id in grouped_tickets.keys() {
            let group_nums = grouped_tickets.get(group_id).unwrap();
            let group_nums_set = HashSet::from_iter(group_nums.clone());
            let remaining: HashSet<_> = group_nums_set.difference(&rule.valid_numbers).collect();
            if remaining.is_empty() {
                entry.insert(group_id);
            }
        }
    }

    let mut results: HashMap<String, usize> = HashMap::new();
    let keys = candidates.keys().map(String::from).collect::<Vec<String>>();
    loop {
        if results.len() == candidates.len() {
            break;
        }

        for key in keys.iter() {
            let key_str = key.to_owned();
            let already_found = {
                results.contains_key(key)
            };


            for c_key in keys.iter() {
                let c_key_str = c_key.to_owned();
                let entry = candidates.entry(c_key.to_owned()).or_default();
                if already_found {
                    // println!("entry: {:?}", entry.clone());
                    entry.remove(results.get(key).unwrap());
                }
                if entry.len() == 1 {
                    results.insert(
                        c_key.to_owned(),
                        *entry.iter().next().unwrap().to_owned(),
                    );
                }
            }
        }
    }
    (tickets_as_nums, results)
}

fn parse_rules_to_struct(rules_str: &str) -> Rule {
    lazy_static! {
        static ref RE_RULES: Regex = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let captures = RE_RULES.captures(&rules_str);
    let groups = captures.unwrap();
    let name = groups.get(1).map_or("", |m| m.as_str()).to_owned();
    let lo1 = groups
        .get(2)
        .map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());
    let hi1 = groups
        .get(3)
        .map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());
    let lo2 = groups
        .get(4)
        .map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());
    let hi2 = groups
        .get(5)
        .map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());

    let mut valid_numbers = HashSet::new();
    for i in lo1..=hi1 {
        valid_numbers.insert(i);
    }

    for i in lo2..=hi2 {
        valid_numbers.insert(i);
    }

    Rule {
        name,
        valid_numbers,
    }
}

struct Rule {
    name: String,
    valid_numbers: HashSet<i64>,
}

fn parse_input(groups: &[String]) -> (Vec<&str>, String, Vec<&str>) {
    let rules = parse_rules(&groups.get(0).unwrap());
    let my_ticket = *groups
        .get(1)
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap();
    let nearby_tickets = parse_nearby_tickets(&groups.get(2).unwrap());

    (rules, my_ticket.to_owned(), nearby_tickets)
}

fn parse_nearby_tickets(lines: &str) -> Vec<&str> {
    lines.split('\n').filter(|s| !s.is_empty()).collect()
}

fn parse_rules(lines: &str) -> Vec<&str> {
    lines.split('\n').filter(|s| !s.is_empty()).collect()
}

fn compute_scanning_error_rate(
    valid_numbers: &HashSet<i64, RandomState>,
    nearby_tickets: &[&str],
) -> i64 {
    let mut scanning_error_rate = 0;
    for line in nearby_tickets {
        if line.starts_with("nearby") {
            continue;
        }

        for part in line.split(',') {
            let n = String::from(part).parse::<i64>().unwrap();
            if !valid_numbers.contains(&n) {
                scanning_error_rate += n;
            }
        }
    }

    scanning_error_rate
}

fn parse_rules_to_set(rules: &Vec<&str>) -> HashSet<i64> {
    let mut valid_numbers = HashSet::new();
    for line in rules {
        let rule = parse_rules_to_struct(&line);
        valid_numbers.extend(rule.valid_numbers);
    }
    valid_numbers
}

#[cfg(test)]
mod tests {
    use crate::day16::{compute_scanning_error_rate, parse_input, parse_nearby_tickets, parse_rules, parse_rules_to_set, part_2, compute_final_result_part_2};

    #[test]
    fn example_1() {
        let lines = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, my_ticket, nearby_tickets) = parse_input(&lines);
        let valid_numbers = parse_rules_to_set(&rules);
        assert_eq!(
            71,
            compute_scanning_error_rate(&valid_numbers, &nearby_tickets)
        )
    }
    #[test]
    fn example_2() {
        let lines = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, my_ticket, nearby_tickets) = parse_input(&lines);
        let valid_numbers = parse_rules_to_set(&rules);
        let (tickets_as_nums, results) = part_2(
            &rules, &my_ticket, &nearby_tickets, &valid_numbers
        );
        let my_ticket_as_num = tickets_as_nums.get(0).unwrap();
        assert_eq!(12, *my_ticket_as_num.get(*results.get("class").unwrap()).unwrap());
        assert_eq!(11, *my_ticket_as_num.get(*results.get("row").unwrap()).unwrap());
        assert_eq!(13, *my_ticket_as_num.get(*results.get("seat").unwrap()).unwrap());

        let result = compute_final_result_part_2(&results, &my_ticket_as_num, "");
        assert_eq!(1716, result);
    }
}
