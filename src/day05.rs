use crate::util;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::Index;

pub fn run() {
    // Get input
    let lines = util::day_input(5);

    let max = lines.iter().map(|l| compute_seat_id(&l[..])).max().unwrap();
    println!("Part 1, max seat ID: {}", max);

    let seats = lines
        .iter()
        .map(|l| compute_seat_id(&l[..]))
        .sorted()
        .collect::<Vec<i32>>();

    for i in 1..seats.len() {
        let pre: &i32 = seats.get(i - 1).unwrap();
        let now: &i32 = seats.get(i).unwrap();
        if now - pre == 2 {
            println!("Part 2: your seat ID: {}", now - 1);
            break;
        }
    }
}

#[derive(Debug)]
struct Range {
    lo: i32,
    hi: i32,
}

fn compute_seat_id(line: &str) -> i32 {
    let mut row = Range { lo: 0, hi: 127 };
    let mut seat = Range { lo: 0, hi: 7 };

    let row_instr = &line[0..6].chars().collect::<Vec<char>>();

    row_instr.iter().for_each(|fb| {
        // println!("in: {:?}", row);
        // println!("instr: {}", fb);

        let mid = row.lo + ((row.hi + 1 - row.lo) / 2);
        match fb {
            &'F' => row.hi = mid - 1,
            &'B' => row.lo = mid,
            _ => panic!("Unknown row instruction"),
        };
        // println!("out: {:?}", row);
    });
    let row = match &line[6..7].chars().nth(0).unwrap() {
        &'F' => row.lo,
        &'B' => row.hi,
        _ => panic!("Unknown row instruction"),
    };
    // println!("row: {}", row);
    // println!();

    let seat_instr = &line[7..9].chars().collect::<Vec<char>>();
    seat_instr.iter().for_each(|lr| {
        // println!("in: {:?}", seat);
        // println!("instr: {}", lr);

        let mid = seat.lo + ((seat.hi + 1 - seat.lo) / 2);
        match lr {
            &'L' => seat.hi = mid - 1,
            &'R' => seat.lo = mid,
            _ => panic!("Unknown seat instruction"),
        };
        // println!("out: {:?}", seat);
    });

    let seat = match &line[9..].chars().nth(0).unwrap() {
        &'L' => seat.lo,
        &'R' => seat.hi,
        _ => panic!("Unknown row instruction"),
    };
    // println!("seat: {}", seat);

    // Formula for seat ID:
    (row * 8) + seat
}

#[cfg(test)]
mod tests {
    use crate::day05::compute_seat_id;

    #[test]
    fn part_1_example_1() {
        assert_eq!(357, compute_seat_id("FBFBBFFRLR"));
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(567, compute_seat_id("BFFFBBFRRR"));
    }

    #[test]
    fn part_1_example_3() {
        assert_eq!(119, compute_seat_id("FFFBBBFRRR"));
    }

    #[test]
    fn part_1_example_4() {
        assert_eq!(820, compute_seat_id("BBFFBBFRLL"));
    }
}
