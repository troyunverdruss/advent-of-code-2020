use crate::util::inputs::day_input;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let lines = day_input(7);

    let all_bags = parse_all_bags(lines);
    let count = count_gold_bags(&all_bags);
    println!("Part 1: bags that can hold gold {}", count);
}

fn count_gold_bags(all_bags: &HashMap<String, HashMap<String, usize>>) -> usize {
    // Now to figure out which ones can hold gold bags ...
    let mut memo: HashMap<String, bool> = HashMap::new();

    let count = all_bags
        .keys()
        .map(|bag| {
            let result = find_gold_bags(bag, &all_bags, &mut memo, 0);
            println!("{} held shiny gold: {}", bag, result);
            println!();
            result
        })
        .filter(|b| b == &true)
        .count();
    count
}

fn parse_all_bags(lines: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
    lazy_static! {
    // vibrant bronze bags contain 3 dim olive bags.
    // shiny teal bags contain 1 posh green bag, 5 pale indigo bags, 1 mirrored purple bag.
        static ref RE_ALL: Regex = Regex::new(r"^(.*) bags contain (.*).$").unwrap();
        static ref RE_HELD_BAGS: Regex = Regex::new(r"^\s*(\d+) (.*) bags?\s*$").unwrap();
    }

    let mut all_bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in lines {
        let captures = RE_ALL.captures(&line);
        let groups = captures.unwrap();
        let holding_bag_color = groups.get(1).map_or("", |m| m.as_str());

        if all_bags.contains_key(holding_bag_color) {
            panic!("Tried to reinsert bag: {}", &holding_bag_color);
        };

        let held_bags: &str = groups.get(2).map_or("", |m| m.as_str());

        let mut all_held_bags = HashMap::new();
        if held_bags != "no other bags" {
            for split in held_bags.split(',') {
                // println!("held bag: {}", &split);
                let bag_details = RE_HELD_BAGS.captures(&split).unwrap();
                let count = bag_details
                    .get(1)
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
                let color = bag_details.get(2).map_or("", |m| m.as_str());
                all_held_bags.insert(color.to_owned(), count);
            }
        };

        all_bags.insert(holding_bag_color.to_owned(), all_held_bags);
    }
    all_bags
}

fn find_gold_bags(
    main_bag: &str,
    all_bags: &HashMap<String, HashMap<String, usize>>,
    memo: &mut HashMap<String, bool>,
    depth: usize,
) -> bool {
    println!("{} Checking bag: {}", depth, main_bag);

    // If we've seen this before, just return that
    if let Some(known) = memo.get(main_bag) {
        println!(" Found result in memo: {} = {}", main_bag, known);
        return known.to_owned();
    } else {
        println!("  memo: haven't found solution for {} yet ", main_bag)
    }

    let mut found_gold = HashSet::new();
    let main_bag_holds_these_bags = all_bags.get(main_bag);
    println!("  holds: {:?}", main_bag_holds_these_bags.unwrap().keys());
    if let Some(held_bags) = main_bag_holds_these_bags {
        for held_bag in held_bags.keys() {
            if held_bag == "shiny gold" {
                found_gold.insert(true);
                println!("  memo: inserting {} contains gold: true", main_bag);
                break;
            } else {
                let result = find_gold_bags(held_bag, all_bags, memo, depth + 1);
                found_gold.insert(result);
            }
        }
    }

    let found_gold_result = found_gold.contains(&true);
    memo.insert(main_bag.to_owned(), found_gold_result);
    found_gold_result
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Bag<'a> {
    color: &'a str,
    count: usize,
}

#[cfg(test)]
mod tests {
    use crate::day07::count_gold_bags;
    use crate::day07::parse_all_bags;

    #[test]
    fn test_1() {
        let lines = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_owned(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_owned(),
            "bright white bags contain 1 shiny gold bag.".to_owned(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_owned(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_owned(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_owned(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_owned(),
            "faded blue bags contain no other bags.".to_owned(),
            "dotted black bags contain no other bags.".to_owned(),
        ];

        let all_bags = parse_all_bags(lines);
        let count = count_gold_bags(&all_bags);
        assert_eq!(4, count);
    }
}
