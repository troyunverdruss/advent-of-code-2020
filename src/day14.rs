use crate::util::inputs::day_input;
use std::collections::HashMap;
use std::io::Bytes;
use lazy_static::lazy_static;
use regex::Regex;

pub fn run() {
    let lines = day_input(14);

    let part1 = part1(&lines);
    println!("Part 1, sum of all non-zero memory: {}", part1);
}

#[derive(Debug)]
struct BitVal{
    bit: usize,
    val: u8
}

fn part1(lines: &[String]) -> i64 {
    let mut all_ones:i64 = 0;
    let base: i64 = 2;

    for pow in 0..36 {
        all_ones |= base.pow(pow);
    }
    let all_ones = all_ones;
    println!("All ones: {}", all_ones);

    let mut mask: Vec<BitVal> = Vec::new();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for line in lines {
        // If we have a new mask, parse it out
        if line.starts_with("mask =") {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            mask = parts
                .get(2)
                .unwrap()
                .chars()
                .rev()
                .into_iter()
                .enumerate()
                .filter(|t| t.1 != 'X')
                .map(|t| BitVal {bit: t.0, val: String::from(t.1).parse::<u8>().unwrap()})
                .collect();
            println!("Mask: {:?}", parts.get(2).unwrap());
            println!("  Parsed: {:?}", mask)
        } else {
            // Otherwise, let's try to write this value
            lazy_static! {
                static ref RE_MEMORY: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            }
            let captures = RE_MEMORY.captures(&line);
            let groups = captures.unwrap();
            let mem_addr:i64 = groups.get(1).map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());
            let new_val:i64 = groups.get(2).map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());



            let mem_val = memory.entry(mem_addr).or_insert(0);
            let mut val = new_val;

            for bitval in &mask {
                match bitval.val {
                    0 => {
                        let modifier = all_ones ^ base.pow(bitval.bit as u32);
                        val &= modifier;
                    },
                    1 => {
                        val |= base.pow(bitval.bit as u32);
                    },
                    _ => unreachable!()
                }
            }
            *mem_val = val;
            let i = 0;
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::day14::part1;

    #[test]
    fn test_example_1() {
        let lines = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
        .split('\n').map(String::from).collect::<Vec<String>>();
        assert_eq!(165, part1(&lines));
    }
}
