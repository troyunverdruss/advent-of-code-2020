use crate::util::inputs::day_input;

pub fn run() {
    let numbers = day_input(9)
        .iter()
        .map(|s| s.parse::<i64>().expect("Could not parse int"))
        .collect::<Vec<i64>>();

    let first_invalid_number = part1(&numbers, 25);
    println!("Part 1: first invalid number: {}", first_invalid_number);

    let encryption_weakness = part2(&numbers, first_invalid_number);
    println!("Part 2: encryption weakness: {}", encryption_weakness);
}

fn part1(numbers: &Vec<i64>, preamble: usize) -> i64 {
    for i in preamble..numbers.len() {
        let mut pre = vec![0; preamble];
        pre.clone_from_slice(&numbers[i - preamble..i]);

        // println!("slice: {:?}", pre);

        let target = numbers[i];
        // println!("target: {:?}", target);

        let candidates = pre
            .iter()
            .filter(|n| *n < &target)
            .copied()
            .collect::<Vec<i64>>();

        // println!("candidates: {:?}", candidates);

        let valid = candidates.iter().any(|c| {
            let pair = target - c;
            let res = if pair == *c {
                false
            } else {
                candidates.contains(&pair)
            };
            // println!("{}+{}={} valid: {}", c, pair, target, res);
            res
        });

        if !valid {
            return numbers[i];
        }
    }
    panic!("Never found a suitable number")
}

fn part2(numbers: &[i64], target_invalid_number: i64) -> i64 {
    let mut running_sum = 0;
    let sums_in_position = numbers
        .iter()
        .map(|n| {
            running_sum += n;
            running_sum
        })
        .collect::<Vec<i64>>();

    // println!("Target number: {}", target_invalid_number);
    // println!("Part 2 numbers    : {:?}", numbers);
    // println!("Part 2 number sums: {:?}", sums_in_position);

    for end in 0..sums_in_position.len() {
        for start in 0..end {
            if sums_in_position[end] - sums_in_position[start] == target_invalid_number {
                // println!("Index {}..{}", start, end);
                // println!("Sum {}..{}", numbers[end], numbers[start + 1]);

                let min = numbers[start + 1..end].iter().min().unwrap();
                let max = numbers[start + 1..end].iter().max().unwrap();

                let sum = min + max;
                // println!("Sum: {}", sum);
                return sum;
            }
        }
    }

    panic!("Never found a valid number!")
}

#[cfg(test)]
mod tests {
    use crate::day09::{part1, part2};

    #[test]
    fn test_input() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(127, part1(&numbers, 5));
        assert_eq!(62, part2(&numbers, 127));
    }
}
