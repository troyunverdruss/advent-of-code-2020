use std::collections::{HashMap, VecDeque};

pub fn run() {
    let starting_numbers: Vec<usize> = vec![0, 14, 1, 3, 7, 9];

    let result = part1(&starting_numbers, 2020);
    println!("Part 1, 2020th number: {}", result);

    let result = part1(&starting_numbers, 30000000);
    println!("Part 2, 2020th number: {}", result);
}

// TODO could maybe do this without storing several values in each key
//  and instead just keep the last seen number "out" of the hash until
//  i'm done with it, older entries stay in the hash and then when i'm done
//  with the calculation i could stuff the "last seen" number value in.
fn part1(starting_numbers: &[usize], target: usize) -> usize {
    let mut numbers = HashMap::new();
    let mut last_number = *starting_numbers.get(0).unwrap();
    for turn in 1..=target {
        if turn <= starting_numbers.len() {
            last_number = *starting_numbers.get(turn - 1).unwrap();

            let entry = numbers.entry(last_number).or_insert_with(VecDeque::new);
            entry.push_front(turn);
            continue;
        }

        // Scoped to release the mutable entry ref
        let current_announced_number = {
            let entry = numbers.get(&last_number).expect("Should always have written an entry by now");
            if entry.len() >= 2 {
                let last_one = entry.get(0).unwrap();
                let last_last_one = entry.get(1).unwrap();

                last_one - last_last_one
            } else {
                0
            }
        };

        let entry = numbers.entry(current_announced_number).or_insert_with(VecDeque::new);
        entry.push_front(turn);
        entry.truncate(2);
        last_number = current_announced_number;
    }

    // 1451 not right
    last_number
}


#[cfg(test)]
mod tests {
    use crate::day15::part1;

    #[test]
    fn test_example_1_1() {
        // assert_eq!(0, part1(&[0,3,6], 1));
        // assert_eq!(3, part1(&[0,3,6], 2));
        // assert_eq!(6, part1(&[0,3,6], 3));
        assert_eq!(0, part1(&[0,3,6], 4));
        assert_eq!(3, part1(&[0,3,6], 5));
        assert_eq!(3, part1(&[0,3,6], 6));
        assert_eq!(1, part1(&[0,3,6], 7));
        assert_eq!(0, part1(&[0,3,6], 8));
        assert_eq!(4, part1(&[0,3,6], 9));
        assert_eq!(0, part1(&[0,3,6], 10));
        assert_eq!(436, part1(&[0,3,6], 2020));
    }
}
