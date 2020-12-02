use crate::util;
use regex::Regex;

#[derive(Debug)]
struct PasswordEntry {
    low: i32,
    high: i32,
    letter: String,
    entry: String,
    matches: i32,
}

impl PasswordEntry {
    pub fn valid_for_part_1(&self) -> bool {
        self.matches >= self.low && self.matches <= self.high
    }

    pub fn valid_for_part_2(&self) -> bool {
        let low = self.entry.chars().nth((self.low - 1) as usize).unwrap();
        let high = self.entry.chars().nth((self.high - 1) as usize).unwrap();

        (low.to_string() == self.letter) ^ (high.to_string() == self.letter)
    }
}

pub fn run() {
    let password_entries: Vec<PasswordEntry> = util::read_lines(2).iter().map(parse_line).collect();

    let count_1 = password_entries
        .iter()
        .filter(|pe| pe.valid_for_part_1())
        .count();

    let count_2 = password_entries
        .iter()
        .filter(|pe| pe.valid_for_part_2())
        .count();

    println!("Part 1: {}", count_1);
    println!("Part 2: {}", count_2);
}

fn parse_line(line: &String) -> PasswordEntry {
    let parts: Vec<String> = line
        .split(|c| c == '-' || c == ':' || c == ' ')
        .map(String::from)
        .filter(|s| s.len() > 0)
        .collect();

    let min = parts.get(0).unwrap().parse().expect("Couldn't get min");
    let max = parts.get(1).unwrap().parse().expect("Couldn't get max");
    let letter = parts.get(2).expect("Couldn't get letter").to_owned();
    let entry = parts
        .get(3)
        .expect("Couldn't get password entry")
        .to_owned();
    let matches = entry.matches(&letter).count() as i32;

    PasswordEntry {
        low: min,
        high: max,
        letter,
        matches,
        entry,
    }
}
