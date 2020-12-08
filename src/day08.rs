use crate::util::inputs::{day_input};
use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let instructions = day_input(8)
        .iter()
        .map(|f|parse_instruction(f))
        .collect::<Vec<Instruction>>();

    let run_result_part_1 = run_code(&instructions);
    println!("Part 1, accumulator just before repeat: {}", run_result_part_1.acc);

    for i in 0..instructions.len() {
        if let Some(instruction) = instructions.get(i) {
            if instruction.op == "acc" {
                continue;
            }

            let copy = instructions.iter().enumerate().map(|(index, inst)| {
                if index == i {
                    if instruction.op == "nop" {
                        Instruction {op: "jmp".to_owned(), val :inst.val}
                    } else if instruction.op == "jmp" {
                        Instruction {op: "nop".to_owned(), val :inst.val}
                    } else {
                        panic!("Should not be possible to 'acc' here")
                    }
                } else {
                    Instruction {op: inst.op.clone(), val: inst.val }
                }
            })
                .collect::<Vec<Instruction>>();

            let run_result_part_2 = run_code(&copy);
            if run_result_part_2.terminated {
                println!("Part 2: {}", run_result_part_2.acc);
                break;
            }
        }
    }

}

struct RunResult {
    acc: i32,
    terminated: bool
}

fn run_code(instructions: &[Instruction]) -> RunResult {
    let mut game_boy = GameBoy { acc: 0, ptr: 0 };
    let mut already_seen_instructions = HashSet::new();
    let mut terminated = false;
    loop {
        let instruction = match instructions.get(game_boy.ptr as usize) {
            None => {
                terminated = true;
                break;
            },
            Some(i) => i,
        };

        if already_seen_instructions.contains(&game_boy.ptr) {
            break;
        } else {
            already_seen_instructions.insert(game_boy.ptr);
        }

        match &instruction.op[..] {
            "acc" => {
                game_boy.acc += instruction.val;
                game_boy.ptr += 1;
            }
            "jmp" => game_boy.ptr += instruction.val,
            "nop" => game_boy.ptr += 1,
            _ => panic!(format!("Unknown instruction: {}", instruction.op)),
        }
    }
    RunResult { acc: game_boy.acc, terminated }
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: (&str, &str) = line.split_whitespace().collect_tuple().unwrap();
    // println!("Parts: {:?}", parts);
    let op = parts.0.to_owned();
    let val = parts.1.parse::<i32>().unwrap().to_owned();

    Instruction { op, val }
}

#[derive(Clone)]
struct Instruction {
    op: String,
    val: i32,
}

struct GameBoy {
    acc: i32,
    ptr: i32,
}
