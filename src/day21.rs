use crate::util::inputs::day_input;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn run() {
    let lines = day_input(21);
    let part1_result = part1(&lines);
    println!("Part 1: {}", part1_result);
}

fn part1(lines: &[String]) -> usize {
    let parsed_lines = lines
        .iter()
        .map(|l| parse_line(l))
        .collect::<Vec<ParsedLine>>();

    let mut allergens_to_possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for pl in &parsed_lines {
        for allergen in &pl.allergens {
            let entry = allergens_to_possible_ingredients
                .entry(allergen.clone())
                .or_insert(HashSet::new());
            if entry.is_empty() {
                entry.extend(pl.ingredients.clone());
            } else {
                let ingredient_set: HashSet<String> = HashSet::from_iter(pl.ingredients.clone());
                let intersection = entry
                    .intersection(&ingredient_set)
                    .map(String::from)
                    .collect::<HashSet<String>>();
                entry.clear();
                entry.extend(intersection);
            }
        }
    }
    let all_ingredients = parsed_lines
        .iter()
        .map(|pl| pl.ingredients.clone())
        .flatten()
        .map(String::from)
        .collect::<Vec<String>>();

    let all_possible_allergen_ingredients = allergens_to_possible_ingredients
        .iter()
        .map(|e| e.1)
        .flatten()
        .map(String::from)
        .collect::<HashSet<String>>();

    all_ingredients
        .iter()
        .filter(|i| !all_possible_allergen_ingredients.contains(*i))
        .count()
}

fn parse_line(line: &str) -> ParsedLine {
    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(r"^(.*)\(contains (.*)\)$").unwrap();
    }
    let captures = RE_LINE.captures(&line);
    let groups = captures.unwrap();

    let ingredients: Vec<String> = groups
        .get(1)
        .map_or(String::new(), |m| String::from(m.as_str()))
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    let allergens: Vec<String> = groups
        .get(2)
        .map_or(String::new(), |m| String::from(m.as_str()))
        .replace(" ", "")
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();

    ParsedLine {
        ingredients,
        allergens,
    }
}

struct ParsedLine {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::day21::part1;

    #[test]
    fn example_1() {
        let lines = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(5, part1(&lines));
    }
}
