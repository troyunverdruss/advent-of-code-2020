// use itertools::Itertools;

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod util;

fn main() {
    let day = 4;

    let mut days: HashMap<i32, fn()> = HashMap::new();
    days.insert(1, day01::run);
    days.insert(2, day02::run);
    days.insert(3, day03::run);
    days.insert(4, day04::run);
    days.insert(5, day05::run);

    days.get(&day).map(|f| f());

    // // Run all the days in order
    // days.iter().sorted().for_each(|e| {
    //     println!("Day {}", e.0);
    //     e.1()
    // })
}
