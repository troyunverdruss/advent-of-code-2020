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

    println!("Part 1: {}", valid_messages.len());

    let part2 = part_2_trick(messages, &mut rules_map);
    println!("Part 2: {}", part2);

    // too low 280
    // too high 297
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
            if number == 8 || number == 11 {
                result.push(format!(" {} ", number));
            } else {
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

#[derive(Clone)]
struct Rule {
    id: i32,
    raw_rule: String,
    expanded_rules: Vec<String>,
}

fn part_2_trick(messages: Vec<String>, mut rules_map: &mut HashMap<i32, Rule>) -> i32 {
    let rule_31 = unroll_from_id(31, &mut rules_map);
    let rule_42 = unroll_from_id(42, &mut rules_map);
    let rule_31: HashSet<String> = HashSet::from_iter(rule_31);
    let rule_42: HashSet<String> = HashSet::from_iter(rule_42);

    let len_31 = rule_31.iter().map(|r| r.len()).min().unwrap();
    assert_eq!(len_31, rule_31.iter().map(|r| r.len()).max().unwrap(),);

    let len_42 = rule_42.iter().map(|r| r.len()).min().unwrap();
    assert_eq!(len_42, rule_42.iter().map(|r| r.len()).max().unwrap(),);

    assert_eq!(len_31, len_42);
    let segment_len = len_31;

    println!("{}", segment_len);

    let mut valid_count = 0;
    for m in messages {
        let m_len = m.len();

        let segments = m_len / len_42;
        if m_len % (len_42) != 0 {
            println!("invalid len: {} m: {}", m_len, m);
        }

        // Rule zero is: 8 11
        // Rule 8 is just 'n' 42
        // Rule 11 is 'n' 42 followed by 'n' 31
        // So it is 2/3 42, then 1/3 31

        let mut valid_segments = vec![];
        let mut checked_rule = vec![];
        let mut front = true;
        println!("{}, {}, {}", &m[..], m_len, segment_len);
        checked_rule.push("42");
        if rule_42.contains(&m[0..segment_len]) {
            valid_segments.push(true);
        } else {
            valid_segments.push(false);
        }

        for i in (segment_len..(m_len-segment_len)).step_by(segment_len) {
            let m_slice = &m[i..(i + segment_len)];
            if front {
                if rule_42.contains(m_slice) {
                    valid_segments.push(true);
                    checked_rule.push("42");
                } else {
                    let remaining_segments = segments - valid_segments.len();
                    if remaining_segments > valid_segments.len()-1{
                        println!("{} segments, {} remaining", segments, remaining_segments);
                        valid_segments.push(false);
                        checked_rule.push("invalid");
                    }
                    front =  false;
                    checked_rule.push("42->");
                }
            }
            if !front {
                checked_rule.push("31");
                if rule_31.contains(m_slice) {
                    valid_segments.push(true);
                } else {
                    valid_segments.push(false);
                }
            }
            // println!("{}..{}: {} [valid: {}]", i, i + len_42 - 1, m_slice, valid_8);
        }
        checked_rule.push("31");
        if rule_31.contains(&m[(m_len-segment_len)..]) {
            valid_segments.push(true);
        } else {
            valid_segments.push(false);
        }

        println!("{:?}", valid_segments);
        println!("{:?}", checked_rule);

        if valid_segments.iter().all(|v| v == &true) {
            // println!("valid: {}", m);
            valid_count += 1;
        } else {
            // println!("invalid: {}", m);
        }

        //
        //
        //
        //
        // let mut valid = true;
        // for i in (0..(2* one_third)).step_by(len_42) {
        //     let m_slice = &m[i..(i + len_42)];
        //     if !rule_42.contains(m_slice) {
        //         valid = false;
        //     }
        //     // println!("{}..{}: {} [valid: {}]", i, i + len_42 - 1, m_slice, valid_8);
        // }
        // for i in ((2* one_third)..(3*one_third)).step_by(len_31) {
        //     let m_slice = &m[i..(i + len_31)];
        //     if !rule_31.contains(m_slice) {
        //         valid = false;
        //     }
        //     // println!("{}..{}: {} [valid: {}]", i, i + len_42 - 1, m_slice, valid_8);
        // }
        //
        //


        // // First 2/3 should be rule 8
        //
        // let mut valid_8 = false;
        // if m_len % len_42 == 0 {
        //     println!("Check 8");
        //     valid_8 = true;
        //     for i in (0..m_len).step_by(len_42) {
        //         let m_slice = &m[i..(i + len_42)];
        //         if !rule_42.contains(m_slice) {
        //             valid_8 = false;
        //         }
        //         println!("{}..{}: {} [valid: {}]", i, i + len_42 - 1, m_slice, valid_8);
        //     }
        // }
        //
        // // Check rule 11
        // let mut valid_11 = false;
        // let len_42_plus_31 = len_42 + len_31;
        // if m_len % (len_42_plus_31) == 0 {
        //     println!("Check 11");
        //     valid_11 = true;
        //     // first half should be 42s
        //     for i in (0..(m_len / 2)).step_by(len_42) {
        //         let m_slice_42 = &m[i..(i + len_42)];
        //         if !rule_42.contains(m_slice_42) {
        //             valid_11 = false;
        //         }
        //         println!("{}..{}: {} [valid: {}]", i, i + len_42 - 1, m_slice_42, valid_11);
        //     }
        //
        //     // second half should be 31s
        //     for i in ((m_len / 2)..m_len).step_by(len_31) {
        //         let m_slice_31 = &m[i..(i + len_31)];
        //         if !rule_31.contains(m_slice_31) {
        //             valid_11 = false;
        //         }
        //         println!("{}..{}: {} [valid: {}]", i, i + len_31 - 1, m_slice_31, valid_11);
        //     }
        // }

        // if valid {
        //     valid_count += 1;
        //     println!("valid: {}", &m[..]);
        // } else {
        //     println!("invalid: {}", &m[..]);
        // }
        println!();
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use crate::day19::{
        count_valid, parse_rules_to_map, parse_sections, part_2_trick, unroll_from_id, Rule,
    };
    use crate::util::inputs::read_lines_split_by_double_newline;
    use itertools::Itertools;
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;

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

    #[test]
    fn print_rule() {
        let sections = read_lines_split_by_double_newline(19);
        let (rules, messages) = parse_sections(sections);

        let mut rules_map = parse_rules_to_map(&rules);

        let valid_count = part_2_trick(messages, &mut rules_map);

        println!("Part 2: {}", valid_count);
    }

    #[test]
    fn example_2_all_valid() {
        let lines = "\
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
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        let sections = lines
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);

        let mut rules_map = parse_rules_to_map(&rules);
        let part2 = part_2_trick(messages, &mut rules_map);

        assert_eq!(12, part2);
    }

    #[test]
    fn example_2_all_invalid() {
        let lines = "\
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

bbbbabbbbaababbababb";
        // abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        // aaaabbaaaabbaaa
        // babaaabbbaaabaababbaabababaaab
        // bbbaabbbaabbbaa
        // babbbbabbbbabbb
        let sections = lines
            .split("\n\n")
            .map(String::from)
            .collect::<Vec<String>>();
        let (rules, messages) = parse_sections(sections);

        let mut rules_map = parse_rules_to_map(&rules);
        let part2 = part_2_trick(messages, &mut rules_map);

        assert_eq!(0, part2);
    }
}
