use crate::util::inputs::day_input;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn run() {
    let lines = day_input(14);

    let part1 = solver(&lines, v1_decoder);
    println!("Part 1, v1 chip, sum of all non-zero memory: {}", part1);

    let part2 = solver(&lines, v2_decoder);
    println!("Part 2, v2 chip, sum of all non-zero memory: {}", part2);
}

#[derive(Debug)]
struct BitVal {
    bit: usize,
    val: u8,
}

static BASE: i64 = 2;

type DecoderChipInstruction = fn(
    memory: &mut HashMap<i64, i64>,
    mem_addr: i64,
    new_val: i64,
    mask: &Vec<BitVal>,
    all_ones: i64,
);

fn solver(lines: &[String], decoder_chip_instruction: DecoderChipInstruction) -> i64 {
    let mut all_ones: i64 = 0;

    for pow in 0..36 {
        all_ones |= BASE.pow(pow);
    }
    let all_ones = all_ones;
    // println!("All ones: {}", all_ones);

    let mut mask: Vec<BitVal> = Vec::new();
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for line in lines {
        // If we have a new mask, parse it out
        if line.starts_with("mask =") {
            mask = parse_mask(line)
        } else {
            // Otherwise, let's try to write this value
            let (mem_addr, new_val) = parse_memory_write_instruction(&line);
            decoder_chip_instruction(&mut memory, mem_addr, new_val, &mask, all_ones);

            let i = 0;
        }
    }
    memory.values().sum()
}

fn v1_decoder(
    memory: &mut HashMap<i64, i64>,
    mem_addr: i64,
    new_val: i64,
    mask: &Vec<BitVal>,
    all_ones: i64,
) {
    let mut val = new_val;

    // V1 uses the mask to modify the value being written
    // So we go through and modify it accordingly here
    for bitval in mask {
        val = change_bit(all_ones, val, bitval)
    }
    let mem_val = memory.entry(mem_addr).or_insert(0);
    *mem_val = val;
}

fn change_bit(all_ones: i64, val: i64, bitval: &BitVal) -> i64 {
    let mut new_val = val;
    match bitval.val {
        0 => {
            let modifier = all_ones ^ BASE.pow(bitval.bit as u32);
            new_val &= modifier;
        }
        1 => {
            new_val |= BASE.pow(bitval.bit as u32);
        }
        _ => unreachable!(),
    };

    new_val
}

fn v2_decoder(
    memory: &mut HashMap<i64, i64>,
    mem_addr: i64,
    new_val: i64,
    mask: &Vec<BitVal>,
    all_ones: i64,
) {
    let mut target_addrs = Vec::new();
    // V2 uses the mask to find multiple addresses to write the value from above into
    // The value is the same for each address we find
    // Let's find all the addresses now ...

    let mapped_mask = mask
        .iter()
        .map(|bv| (bv.bit, bv.val))
        .collect::<HashMap<usize, u8>>();

    // Figure out which bits are floating (ones not explicitly in our mask)
    let mut floating_bits = Vec::new();
    for bit in 0..36 {
        if mapped_mask.get(&bit).is_none() {
            floating_bits.push(bit);
        }
    }

    // Overwrite all bits set to 1 in the mask with a 1 value
    // println!("mem_addr: {:b}", mem_addr);
    let mut base_addr = mem_addr;
    mask.iter().filter(|bv| bv.val == 1).for_each(|bv| {
        base_addr |= BASE.pow(bv.bit as u32);
    });
    let base_addr = base_addr;
    // println!("base addr: {:b}", base_addr);


    for permutation in find_permutations(floating_bits.len()) {
        let mut new_addr = base_addr;
        for (index, value) in floating_bits.iter().enumerate() {
            let bit_val = BitVal {
                bit: *floating_bits.get(index).unwrap(),
                val: *permutation.get(index).unwrap(),
            };
            // println!("bit_val: {:?}", bit_val);
            new_addr = change_bit(
                all_ones,
                new_addr,
                &bit_val,
            );
            // println!("  Changed 1 bit: {:b}", new_addr);
        }
        // println!("New addr: {:b}", new_addr);
        target_addrs.push(new_addr);
    }

    // Write to all the addresses
    for addr in target_addrs {
        // println!("Inserting {} into {}", new_val, addr);
        let mem_val = memory.entry(addr).or_insert(0);
        *mem_val = new_val;
    }
}

fn find_permutations(length: usize) -> Vec<Vec<u8>> {
    if length == 1 {
        return vec![vec![0], vec![1]];
    }

    let children = find_permutations(length - 1);
    let mut result: Vec<Vec<u8>> = Vec::new();
    for lead_bit in 0..=1 {
        for combo in &children {
            let mut new_result = vec![lead_bit];
            new_result.extend(combo);
            result.push(new_result);
        }
    }

    result
}

fn parse_mask(line: &String) -> Vec<BitVal> {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let mask: Vec<BitVal> = parts
        .get(2)
        .unwrap()
        .chars()
        .rev()
        .into_iter()
        .enumerate()
        .filter(|t| t.1 != 'X')
        .map(|t| BitVal {
            bit: t.0,
            val: String::from(t.1).parse::<u8>().unwrap(),
        })
        .collect();
    // println!("Mask: {:?}", parts.get(2).unwrap());
    // println!("  Parsed: {:?}", mask);

    mask
}

fn parse_memory_write_instruction(line: &String) -> (i64, i64) {
    lazy_static! {
        static ref RE_MEMORY: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }
    let captures = RE_MEMORY.captures(&line);
    let groups = captures.unwrap();
    let mem_addr: i64 = groups
        .get(1)
        .map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());
    let new_val: i64 = groups
        .get(2)
        .map_or(-1, |m| String::from(m.as_str()).parse::<i64>().unwrap());

    (mem_addr, new_val)
}

#[cfg(test)]
mod tests {
    use crate::day14::{find_permutations, solver, v1_decoder, v2_decoder};

    #[test]
    fn test_example_1() {
        let lines = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(165, solver(&lines, v1_decoder));
    }

    #[test]
    fn test_example_2_1() {
        let lines = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(208, solver(&lines, v2_decoder));
    }

    #[test]
    fn test_permutations() {
        let just_one = find_permutations(1);
        assert_eq!(vec![vec![0], vec![1]], just_one);
        let two = find_permutations(2);
        assert_eq!(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]], two);
    }
}
