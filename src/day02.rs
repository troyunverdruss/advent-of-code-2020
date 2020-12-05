use crate::util;

#[derive(Debug)]
struct PasswordEntry {
    low: usize,
    high: usize,
    letter: char,
    entry: String,
}

impl PasswordEntry {
    fn valid_for_part_1(&self) -> bool {
        let count = self
            .entry
            .chars()
            .into_iter()
            .filter(|c| *c == self.letter)
            .count();

        self.low <= count && count <= self.high
    }

    fn valid_for_part_2(&self) -> bool {
        let indexes = vec![self.low - 1, self.high - 1];
        let char_matches = self
            .entry
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(i, _)| indexes.contains(i))
            .map(|(_, c)| c == self.letter)
            .collect::<Vec<bool>>();

        char_matches[0] ^ char_matches[1]
    }
}

pub fn run() {
    let password_entries: Vec<PasswordEntry> = util::day_input(2).iter().map(parse_line).collect();

    let count_1 = password_entries
        .iter()
        .filter(|e| e.valid_for_part_1())
        .count();

    let count_2 = password_entries
        .iter()
        .filter(|e| e.valid_for_part_2())
        .count();

    println!("Part 1: {}", count_1);
    println!("Part 2: {}", count_2);
}

fn parse_line(line: &String) -> PasswordEntry {
    let parts: Vec<String> = line
        .replace("-", " ")
        .replace(":", " ")
        .split(" ")
        .filter(|e| !e.is_empty())
        .map(String::from)
        .collect();

    assert_eq!(4, parts.len(), "Couldn't parse entry!");

    let min = parts.get(0).unwrap().parse().unwrap();
    let max = parts.get(1).unwrap().parse().unwrap();
    let letter = parts.get(2).unwrap().chars().nth(0).unwrap();
    let entry = parts.get(3).unwrap().to_owned();

    PasswordEntry {
        low: min,
        high: max,
        letter,
        entry,
    }
}
