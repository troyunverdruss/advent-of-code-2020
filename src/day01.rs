use crate::util::inputs;
use combinations::Combinations;

pub fn run() {
    println!("Part 1: {}", solver(2).unwrap_or_default());
    println!("Part 2: {}", solver(3).unwrap_or_default());
}

// Solves in O(n^p) time, where p = picks
fn solver(pick_entries: usize) -> Option<i32> {
    let input: Vec<i32> = inputs::day_input(1)
        .iter()
        .map(|s| s.parse().expect("unable to parse int"))
        .collect();

    let combos = Combinations::new(input, pick_entries);

    let mut result = Option::None;
    for combo in combos {
        if combo.iter().sum::<i32>() == 2020 {
            result = Some(combo.iter().product());
        }
    }

    result
}

// TODO Solve in linear time
// TODO Solve with 3Sum algorithm to improve pick=3 performance

#[cfg(test)]
mod tests {
    #[test]
    fn test_i32_product() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(120, nums.iter().product());
    }
}
