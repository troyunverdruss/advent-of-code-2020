use crate::day18::Value::{Number, Operator, Parens};
use crate::util::inputs::day_input;
use lazy_static::lazy_static;
use regex::Regex;
use std::any::Any;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let lines = day_input(18);

    let part1_result: i64 = lines.iter().map(|l| part_1_solver(&l)).sum();
    println!("Part 1: {}", part1_result);

    let lines = day_input(18);
    let part2_result = part2(&lines);

    println!("Part 2: {}", part2_result);
}

fn part2(lines: &Vec<String>) -> i64 {
    let part2_result: i64 = lines
        .iter()
        .map(|l| part_2_reducer(l))
        .map(|s| {
            let v = s.parse::<i64>().unwrap();
            println!("{}", v);
            v
        })
        .sum();
    part2_result
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Ord, PartialOrd)]
enum Value {
    Number(i64),
    Operator(char),
    Parens(char),
}

impl Value {
    fn from(str: &str) -> Value {
        match str {
            "(" => Parens('('),
            ")" => Parens(')'),
            "*" => Operator('*'),
            "+" => Operator('+'),
            _ => Number(String::from(str).parse::<i64>().unwrap()),
        }
    }

    fn is_number(&self) -> Option<i64> {
        match self {
            Number(n) => Some(*n),
            _ => None,
        }
    }

    fn is_operator(&self) -> Option<char> {
        match self {
            Operator(o) => Some(*o),
            _ => None,
        }
    }

    fn left_parens(&self) -> bool {
        match self {
            Parens(p) => p == &'(',
            _ => false,
        }
    }

    fn right_parens(&self) -> bool {
        match self {
            Parens(p) => p == &')',
            _ => false,
        }
    }
}

fn part_1_solver(line: &str) -> i64 {
    let values = line
        .replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|v| Value::from(v))
        .collect::<Vec<Value>>();

    let mut stack: VecDeque<Value> = VecDeque::new();
    for value in values {
        println!("Stack: {:?}", stack);
        println!("Value: {:?}", value);
        match value {
            Number(number_val) => handle_number(&mut stack, number_val),
            Operator(_) => stack.push_back(value),
            Parens(p) => {
                if p == '(' {
                    stack.push_back(Parens('('))
                } else {
                    let v2 = stack.pop_back();
                    assert!(v2.is_some());
                    let v2 = v2.unwrap();
                    let v2 = v2.is_number();
                    assert!(v2.is_some());
                    let p2 = stack.pop_back();
                    assert!(p2.is_some() && p2.unwrap().left_parens());

                    handle_number(&mut stack, v2.unwrap());
                }
            }
        }
    }
    assert!(!stack.is_empty());
    assert_eq!(1, stack.len());
    stack.front().unwrap().is_number().unwrap()
}

fn handle_number(stack: &mut VecDeque<Value>, number_val: i64) {
    if stack.is_empty() {
        stack.push_back(Number(number_val))
    } else {
        let last = stack.back().unwrap();

        if last.left_parens() {
            stack.push_back(Number(number_val));
            return;
        }

        let op = stack.pop_back();
        assert!(op.is_some());
        let op = op.unwrap();
        let op = op.is_operator().unwrap();

        let v2 = stack.pop_back();
        assert!(v2.is_some());
        let v2 = v2.unwrap();
        let v2 = v2.is_number().unwrap();

        let result = match op {
            '*' => number_val * v2,
            '+' => number_val + v2,
            _ => unreachable!(),
        };
        stack.push_back(Number(result));
    }
}

fn part_2_reducer(line: &str) -> String {
    lazy_static! {
        static ref RE_PARENS: Regex = Regex::new(r"(\([\d+*]+\))").unwrap();
        static ref RE_ADD: Regex = Regex::new(r"(\d+)\+(\d+)").unwrap();
        static ref RE_MUL: Regex = Regex::new(r"(\d+)\*(\d+)").unwrap();
    }

    let mut line_copy = line.replace(" ", "");

    while RE_PARENS.is_match(&line_copy)
        || RE_ADD.is_match(&line_copy)
        || RE_MUL.is_match(&line_copy)
    {
        println!("Line: {}", line_copy);
        if RE_PARENS.is_match(&line_copy) {
            let captures = RE_PARENS.captures(&line_copy);
            let match_str = captures.unwrap().get(1).map_or("", |m| m.as_str());
            let match_str_2 = match_str.replace("(", "").replace(")", "");

            let reduced = part_2_reducer(&match_str_2);
            line_copy = line_copy.replacen(match_str, &reduced, 1);
        } else if RE_ADD.is_match(&line_copy) {
            let captures = RE_ADD.captures(&line_copy);
            let groups = captures.unwrap();
            let d1 = groups
                .get(1)
                .map_or(0, |m| String::from(m.as_str()).parse::<i64>().unwrap());
            let d2 = groups
                .get(2)
                .map_or(0, |m| String::from(m.as_str()).parse::<i64>().unwrap());
            let sum = d1 + d2;
            line_copy = line_copy.replacen(&format!("{}+{}", d1, d2), &format!("{}", sum), 1);
        } else {
            // multiply
            let captures = RE_MUL.captures(&line_copy);
            let groups = captures.unwrap();
            let d1 = groups
                .get(1)
                .map_or(0, |m| String::from(m.as_str()).parse::<i64>().unwrap());
            let d2 = groups
                .get(2)
                .map_or(0, |m| String::from(m.as_str()).parse::<i64>().unwrap());
            let product = d1 * d2;
            line_copy = line_copy.replacen(&format!("{}*{}", d1, d2), &format!("{}", product), 1);
        }
    }

    println!("Result: {}", line_copy);
    println!();
    line_copy
}

#[cfg(test)]
mod tests {
    use crate::day18::{part2, part_1_solver, part_2_reducer};

    #[test]
    fn example_1_1() {
        let line = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(71, part_1_solver(line))
    }

    #[test]
    fn example_1_2() {
        let line = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(51, part_1_solver(line))
    }

    #[test]

    fn example_1_3() {
        let line = "2 * 3 + (4 * 5)";
        assert_eq!(26, part_1_solver(line))
    }

    #[test]
    fn example_1_4() {
        let line = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(437, part_1_solver(line))
    }

    #[test]
    fn example_1_5() {
        let line = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(12240, part_1_solver(line))
    }
    #[test]
    fn example_1_6() {
        let line = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(13632, part_1_solver(line))
    }

    #[test]
    fn example_2_1() {
        let line = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(231, part_2_reducer(line).parse::<i64>().unwrap())
    }

    #[test]
    fn example_2_2() {
        let line = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(51, part_2_reducer(line).parse::<i64>().unwrap())
    }

    #[test]

    fn example_2_3() {
        let line = "2 * 3 + (4 * 5)";
        assert_eq!(46, part_2_reducer(line).parse::<i64>().unwrap())
    }

    #[test]
    fn example_2_4() {
        let line = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(1445, part_2_reducer(line).parse::<i64>().unwrap())
    }

    #[test]
    fn example_2_5() {
        let line = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(669060, part_2_reducer(line).parse::<i64>().unwrap())
    }
    #[test]
    fn example_2_6() {
        let line = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(23340, part_2_reducer(line).parse::<i64>().unwrap())
    }

    #[test]
    fn part_2_1() {
        let line = "(3 + 9 + (9 * 7) + 9) + 5 + 5";
        assert_eq!(94, part_2_reducer(line).parse::<i64>().unwrap())
    }

    #[test]
    fn part_2_2() {
        let line = "3 + 9 + 63 + 9";
        assert_eq!(84, part_2_reducer(line).parse::<i64>().unwrap());
        let line = "63 + 9 + 3 + 9";
        assert_eq!(84, part_2_reducer(line).parse::<i64>().unwrap());
    }



    #[test]
    fn test_part2() {
        let lines = "\
1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(231 + 23340 + 669060 + 1445 + 46 + 51, part2(&lines));
    }
}
