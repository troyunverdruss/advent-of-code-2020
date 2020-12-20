use crate::util::inputs::read_lines_split_by_double_newline;
use itertools::Itertools;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Add;

pub fn run() {
    let sections = read_lines_split_by_double_newline(19);
    let (rules, messages) = parse_sections(sections);

    let mut rules_map = parse_rules_to_map(&rules);

    let zero_rules = unroll_from_id(0, &mut rules_map);
    let valid_messages = count_valid(&zero_rules, &messages);

    println!("Part 1: {}", valid_messages.len())
}

fn count_valid(rules: &Vec<String>, messages: &Vec<String>) -> Vec<String> {
    let rules_set: HashSet<String> = HashSet::from_iter(rules.clone());
    messages
        .iter()
        .filter(|m| rules_set.contains(m.to_owned()))
        .map(String::from)
        .collect::<Vec<String>>()
}

fn parse_sections(sections: Vec<String>) -> (Vec<String>, Vec<String>) {
    let rules = sections
        .get(0)
        .unwrap()
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();
    let messages = sections
        .get(1)
        .unwrap()
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();
    (rules, messages)
}

fn parse_rules_to_map(rules: &[String]) -> HashMap<i32, Rule, RandomState> {
    let rules: Vec<Rule> = rules.iter().map(|l| parse_rule(l)).collect();
    let rules_map: HashMap<i32, Rule> = rules.iter().map(|r| (r.id, r.clone())).collect();
    rules_map
}

fn unroll_from_id(start: i32, mut rules_map: &mut HashMap<i32, Rule, RandomState>) -> Vec<String> {
    let start = rules_map.get(&start).unwrap().to_owned();
    if !start.expanded_rules.is_empty() {
        return start.expanded_rules;
    }

    let branches = start
        .raw_rule
        .split('|')
        .map(String::from)
        .collect::<Vec<String>>();
    let mut results: Vec<String> = vec![];

    for branch in branches {
        let mut result: Vec<String> = vec![];
        for number in branch
            .split_whitespace()
            .map(|v| String::from(v).parse::<i32>().unwrap())
        {
            let unrolled_results = unroll_from_id(number, &mut rules_map);

            if result.is_empty() {
                result.extend_from_slice(&unrolled_results[..]);
            } else {
                let result_clone = result.clone();
                result.clear();
                for unrolled_result in &unrolled_results {
                    for r in &result_clone {
                        result.push(r.to_owned().add(unrolled_result))
                    }
                }
            }
        }

        results.extend(result)
    }

    rules_map.insert(
        start.id,
        Rule {
            id: start.id,
            raw_rule: start.raw_rule.to_owned(),
            expanded_rules: results.clone(),
        },
    );

    results
}

fn parse_rule(line: &str) -> Rule {
    let parts = line.split(':').map(String::from).collect::<Vec<String>>();
    let id = parts.get(0).unwrap().parse::<i32>().unwrap();
    let raw_rule = parts.get(1).unwrap().trim_start().to_owned();
    let expanded_rules = if raw_rule == "\"a\"" {
        vec!["a".to_owned()]
    } else if raw_rule == "\"b\"" {
        vec!["b".to_owned()]
    } else {
        vec![]
    };

    Rule {
        id,
        raw_rule,
        expanded_rules,
    }
}

#[derive(Clone)]
struct Rule {
    id: i32,
    raw_rule: String,
    expanded_rules: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::day19::{count_valid, parse_rules_to_map, parse_sections, unroll_from_id};

    #[test]
    fn example_1() {
        let sections = "\
0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

a
b"
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);
        let mut rules_map = parse_rules_to_map(&rules);
        let zero_rules = unroll_from_id(0, &mut rules_map);
        assert_eq!(2, zero_rules.len());
        assert_eq!("aab", zero_rules.get(0).unwrap());
        assert_eq!("aba", zero_rules.get(1).unwrap());
    }

    #[test]
    fn example_1_full() {
        let sections = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);
        let mut rules_map = parse_rules_to_map(&rules);
        let zero_rules = unroll_from_id(0, &mut rules_map);
        assert_eq!(2, count_valid(&zero_rules, &messages).len())
    }
}
