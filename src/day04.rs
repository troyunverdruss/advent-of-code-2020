use crate::util;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    part_1();
    part_2();
}

fn part_1() {
    let passport_data = collect_all_passport_data();

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    // Take all the raw data, convert to passport hashes, then
    // try and get all the required fields, make sure they're all
    // present, and then count the ones that made it through
    let valid_passports = passport_data
        .iter()
        .map(|pd| to_passport(&pd))
        .map(|p| {
            required_fields
                .iter()
                .map(|r| p.get(r))
                .all(|f| f.is_some())
        })
        .filter(|b| b == &true)
        .count();

    println!("Part 1, valid passports: {}", valid_passports);
}

fn part_2() {
    let passport_data = collect_all_passport_data();

    let valid_passports = passport_data
        .iter()
        .map(|pd| to_passport(&pd))
        .filter(|p| part_2_valid_passports(p))
        .count();

    println!("Part 2, valid passports: {}", valid_passports);
}

fn to_passport(data: &str) -> HashMap<&str, String> {
    let mut passport = HashMap::new();

    data.split_ascii_whitespace()
        .filter(|f| !f.is_empty())
        .for_each(|f| {
            let parts: Vec<&str> = f.split(":").collect();
            passport.insert(parts[0], parts[1].to_owned());
        });

    passport
}

fn part_2_valid_passports(passport: &HashMap<&str, String>) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr = valid_byr(&passport.get("byr"));
    // let byr = passport
    //     .get("byr")
    //     .map(|v| String::from(v).parse::<i32>())
    //     .filter(|v| v.is_ok())
    //     .map(|v| v.unwrap())
    //     .map(|v| 1920 < v && v < 2002)
    //     .filter(|v| v == &true);

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr = valid_iyr(&passport.get("iyr"));

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr = valid_eyr(&passport.get("eyr"));

    let hgt = valid_hgt(&passport.get("hgt"));

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let hcl = valid_hcl(&passport.get("hcl"));

    //     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let ecl = valid_ecl(&passport.get("ecl"));

    //     pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid = valid_pid(&passport.get("pid"));

    // 152 too high
    // 88 is not correct
    // 151 is not correct
    byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn valid_byr(byr: &Option<&String>) -> bool {
    byr.map(|v| v.parse::<i32>().unwrap_or(0))
        .map(|val| (1920 <= val) && (val <= 2002))
        .unwrap_or(false)
}

fn valid_iyr(iyr: &Option<&String>) -> bool {
    if iyr.is_none() {
        return false;
    }
    let val = iyr.unwrap().parse::<i32>().unwrap();

    (2010 <= val) && (val <= 2020)
}

fn valid_eyr(eyr: &Option<&String>) -> bool {
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    if eyr.is_none() {
        return false;
    }
    let val = eyr.unwrap().parse::<i32>().unwrap();

    (2020 <= val) && (val <= 2030)
}

fn valid_hgt(hgt: &Option<&String>) -> bool {
    if hgt.is_none() {
        return false;
    }
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.

    let height = String::from(hgt.unwrap());
    if !height.contains("in") && !height.contains("cm") {
        return false;
    }

    let len = height.len();
    let val = height[..len - 2].parse::<i32>().unwrap();
    let unit = &height[len - 2..];

    let valid = match unit {
        "cm" => (150 <= val) && (val <= 193),
        "in" => (59 <= val) && (val <= 76),
        _ => false,
    };

    valid
}

fn valid_hcl(hcl: &Option<&String>) -> bool {
    if hcl.is_none() {
        return false;
    }

    let hair_color = String::from(hcl.unwrap());

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    RE.is_match(&hair_color)
}

fn valid_ecl(ecl: &Option<&String>) -> bool {
    if ecl.is_none() {
        return false;
    }

    let eye_color = &ecl.unwrap()[..];

    //     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.

    let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    valid.contains(&eye_color)
}

fn valid_pid(pid: &Option<&String>) -> bool {
    if pid.is_none() {
        return false;
    }

    let passport_id = String::from(pid.unwrap());

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    RE.is_match(&passport_id)
}

fn collect_all_passport_data() -> Vec<String> {
    let lines = util::day_input(4);

    let mut passport_data: Vec<String> = Vec::new();
    let mut single_passport: String = String::new();

    for line in lines {
        if line.is_empty() {
            passport_data.push(single_passport.clone());
            single_passport.clear();
        }

        single_passport.push_str(&line);
        single_passport.push_str(" ");
    }

    // Get the last one that was being built in progress
    passport_data.push(single_passport);
    passport_data
}

