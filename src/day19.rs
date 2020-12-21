use crate::util::inputs::read_lines_split_by_double_newline;
use itertools::{min, Itertools};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Add;

pub fn run() {
    let sections = read_lines_split_by_double_newline(19);
    let (rules, messages) = parse_sections(sections);

    let valid_segments = find_valid_segments(&messages);

    let mut rules_map = parse_rules_to_map(&rules);
    let max_depth = messages.iter().map(|m| m.len()).max().unwrap();
    let zero_rules = unroll_from_id(0, &mut rules_map, max_depth + 1, max_depth, &valid_segments);
    let valid_messages = count_valid(&zero_rules, &messages);

    println!("Part 1: {}", valid_messages.len());

    let rules = replace_8_11(&rules);
    let mut rules_map = parse_rules_to_map(&rules);
    let max_depth = messages.iter().map(|m| m.len()).max().unwrap();
    let zero_rules = unroll_from_id(0, &mut rules_map, max_depth + 1, max_depth, &valid_segments);
    let valid_messages = count_valid(&zero_rules, &messages);

    println!("Part 2: {}", valid_messages.len());
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
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    (rules, messages)
}

fn parse_rules_to_map(rules: &[String]) -> HashMap<i32, Rule, RandomState> {
    let rules: Vec<Rule> = rules.iter().map(|l| parse_rule(l)).collect();
    let rules_map: HashMap<i32, Rule> = rules.iter().map(|r| (r.id, r.clone())).collect();
    rules_map
}

fn unroll_from_id(
    start: i32,
    mut rules_map: &mut HashMap<i32, Rule, RandomState>,
    max_depth: usize,
    max_length: usize,
    valid_segments: &HashSet<String>,
) -> Vec<String> {
    let start = rules_map.get(&start).unwrap().to_owned();
    if !start.expanded_rules.is_empty() {
        return start.expanded_rules;
    }
    if max_depth == 0 || max_length == 0 {
        return vec!["".to_owned()];
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
            let current_shortest = if let Some(min) = result.iter().map(|r| r.len()).min() {
                min
            } else {
                0
            };
            let unrolled_results = unroll_from_id(
                number,
                &mut rules_map,
                max_depth - 1,
                max_length - 1,
                valid_segments,
            );

            if result.is_empty() {
                for unrolled_result in &unrolled_results {
                    if valid_segments.contains(unrolled_result) {
                        result.push(unrolled_result.clone());
                    }
                }
            } else {
                let result_clone = result.clone();
                result.clear();
                for unrolled_result in &unrolled_results {
                    for r in &result_clone {
                        if valid_segments.contains(unrolled_result) {
                            result.push(r.to_owned().add(unrolled_result))
                        }
                    }
                }
            }
            // if unrolled_results.iter().map(|r| r.len()).max().unwrap() > max_length {
            //     break;
            // }
        }

        results.extend(result)
    }

    rules_map.insert(
        start.id,
        Rule {
            id: start.id,
            raw_rule: start.raw_rule.replace("\"", "").to_owned(),
            expanded_rules: results.clone(),
        },
    );

    results
}

fn is_valid_message(message: &str, start: i32, rules_map: &HashMap<i32, Rule>) -> bool {
    if let Some(result) = process(message, start, rules_map, 0) {
        println!("result: {}", result);
        println!();
        return result == message;
    }

    false
}

fn debug(msg: String, level: usize) {
    // if level ==0 {
    //     println!("{}: {:indent$}{}", level, " ", msg, indent = level);
        println!("{:indent$}{} {}", " ", level, msg, indent = level);

    // }
}

fn process(
    message: &str,
    start: i32,
    rules_map: &HashMap<i32, Rule>,
    level: usize,
) -> Option<String> {
    debug(format!("message: {}", message), level);

    //     // If our passed in message is empty then we never found a match
    //     // and it's time to bail?
    //     if message.is_empty() {
    // return None;
    //     }

    // If the message matches our raw rule then we've found "a" or "b"
    // so return true
    let rule = rules_map.get(&start).expect("No rule??");
    // if message == rule.raw_rule {
    //     return Some(rule.raw_rule.to_owned());
    // }
    if rule.raw_rule == "a" || rule.raw_rule == "b" {
        return if message.starts_with(&rule.raw_rule) {
            // println!("{:indent$} Matched start with rule: {}, match: {}", " ", rule.id, &rule.raw_rule, indent=level);
            debug(format!("-> return: Some({})", &rule.raw_rule), level);

            Some(rule.raw_rule.to_owned())
        } else {
            // println!("{:indent$} No initial match for rule: {}", " ", rule.id, indent=level);
            debug(format!("-> return: None"), level);
            None
        };
    }

    let branches = rule
        .raw_rule
        .split('|')
        .map(String::from)
        .collect::<Vec<String>>();
    let mut message_copy = message.to_owned();
    let mut matched_message = String::new();
    for branch in branches {
        // println!("{:indent$} Rule: {}, branch: {}", " ", rule.id, branch, indent=level);
        message_copy = message.to_owned();
        matched_message = String::new();
        let branch_numbers: Vec<i32> = branch
            .split_whitespace()
            .map(|v| String::from(v).parse::<i32>().unwrap())
            .collect();

        let mut missed_something = false;
        for number in branch_numbers {
            debug(format!(" number: {}", number), level);
            debug(format!("  message copy: {}", message_copy), level);
            debug(format!("  matched message: {}", matched_message), level);
            let result = process(&message_copy.clone(), number, rules_map, level + 1);
            if result.is_none() {
                debug(format!("  result is none; break"), level);
                missed_something = true;
                break;
            }

            let result = result.unwrap();
            if result.len() > message_copy.len() {
                debug(format!("  result is too long; break"), level);
                missed_something = true;
                break;
            }

            if !message_copy.starts_with(&result) {
                debug(
                    format!("  message copy doesn't start with result; break"),
                    level,
                );
                missed_something = true;
                break;
            }

            debug(format!("  updating message copy and matched messae"), level);
            message_copy = message_copy.replacen(&result, "", 1);
            matched_message = matched_message.add(&result);
            debug(format!("  message copy: {}", message_copy), level);
            debug(format!("  matched message: {}", matched_message), level);
        }

        debug(String::new(), level);
        if !matched_message.is_empty() && !missed_something {
            debug(
                format!(" Matched start of message: {}", matched_message),
                level,
            );
            return Some(matched_message);
        }
    }

    debug(format!("  message copy: {}", message_copy), level);
    debug(format!("  matched message: {}", matched_message), level);
    debug(format!(" No match 2"), level);
    debug(format!(" -> return: None"), level);
    None
}

fn parse_rule(line: &str) -> Rule {
    let parts = line.split(':').map(String::from).collect::<Vec<String>>();
    let id = parts.get(0).unwrap().parse::<i32>().unwrap();
    let raw_rule = parts
        .get(1)
        .unwrap()
        .trim_start()
        .replace("\"", "")
        .to_owned();
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

fn replace_8_11(rules: &Vec<String>) -> Vec<String> {
    let mut new_rules: Vec<String> = rules
        .iter()
        .filter(|r| !r.starts_with("8:") && !r.starts_with("11:"))
        .map(String::from)
        .collect();

    new_rules.push("8: 42 | 42 8".to_owned());
    new_rules.push("11: 42 31 | 42 11 31".to_owned());

    new_rules
}

fn find_valid_segments(messages: &Vec<String>) -> HashSet<String> {
    let mut results = HashSet::new();
    for m in messages {
        for start in 0..=(m.len() - 1) {
            for end in ((start)..=(m.len() - 1)).rev() {
                results.insert(m[start..=end].to_owned());
            }
        }
    }

    results
}

#[derive(Clone)]
struct Rule {
    id: i32,
    raw_rule: String,
    expanded_rules: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::day19::{
        count_valid, find_valid_segments, is_valid_message, parse_rules_to_map, parse_sections,
        replace_8_11, unroll_from_id,
    };
    use itertools::max;
    use crate::util::inputs::read_lines_split_by_double_newline;

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
        let valid_segments = find_valid_segments(&messages);
        let mut rules_map = parse_rules_to_map(&rules);
        let max_depth = messages.iter().map(|m| m.len()).max().unwrap();
        let zero_rules =
            unroll_from_id(0, &mut rules_map, max_depth + 1, max_depth, &valid_segments);
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
        let valid_segments = find_valid_segments(&messages);
        let mut rules_map = parse_rules_to_map(&rules);
        let max_depth = messages.iter().map(|m| m.len()).max().unwrap();
        let zero_rules =
            unroll_from_id(0, &mut rules_map, max_depth + 1, max_depth, &valid_segments);
        assert_eq!(2, count_valid(&zero_rules, &messages).len())
    }

    #[test]
    fn test_simple_valid_rule() {
        let sections = "\
0: 1 2
1: 2 1 | 3
2: \"a\"
3: 2

aaa"
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);
        let mut rules_map = parse_rules_to_map(&rules);
        let valid_count = messages
            .iter()
            .filter(|m| is_valid_message(m, 0, &rules_map))
            .count();
        assert_eq!(1, valid_count)
    }

    #[test]
    fn example_1_checking_for_valid_messages_only() {
        let sections = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

aaabbb
abbbab
ababbb
bababa
aaaabbb"
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);
        let rules_map = parse_rules_to_map(&rules);
        let valid_count = messages
            .iter()
            .filter(|m| is_valid_message(m, 0, &rules_map))
            .count();
        assert_eq!(2, valid_count)
    }

    #[test]
    fn example_2_full_like_part_1() {
        let sections = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);
        let valid_segments = find_valid_segments(&messages);
        let rules_map = parse_rules_to_map(&rules);
        let valid_count = messages
            .iter()
            .filter(|m| is_valid_message(m, 0, &rules_map))
            .count();
        assert_eq!(3, valid_count);
    }

    #[test]
    fn example_2_full_infinite() {
        let sections = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);
        let rules = replace_8_11(&rules);
        let rules_map = parse_rules_to_map(&rules);

        let expected = "\
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
            .split('\n')
            .collect::<Vec<&str>>();

        for m in expected {
            println!("expected good message: {}", m);
            assert!(is_valid_message(m, 0, &rules_map));
        }

        let valid_count = messages
            .iter()
            .filter(|m| is_valid_message(m, 0, &rules_map))
            .count();
        assert_eq!(12, valid_count);

        //
        //         let valid = count_valid(&zero_rules, &messages);
        //
        //         for e in expected {
        //             assert!(valid_segments.contains(e));
        //             println!("Expected {}, found: {}", e, valid.contains(&e.to_owned()));
        //         }

        // assert_eq!(12, valid.len())
    }

    #[test]
    fn test_valid_segments() {
        let s = String::from("abcd");
        let r = find_valid_segments(&vec![s]);
        assert_eq!(10, r.len());
    }

    #[test]
    fn print_rule() {
        let sections = read_lines_split_by_double_newline(19);
        let (rules, messages) = parse_sections(sections);
        let valid_segments = find_valid_segments(&messages);
        let mut rules_map = parse_rules_to_map(&rules);
        let max_depth = messages.iter().map(|m| m.len()).max().unwrap();
        let rules = unroll_from_id(42, &mut rules_map, max_depth + 1, max_depth, &valid_segments);
        println!("{:?}", rules);
    }
}
