// use itertools::Itertools;

use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod util;

fn main() {
    let _day = 15;

    let mut days: HashMap<i32, fn()> = HashMap::new();
    days.insert(1, day01::run);
    days.insert(2, day02::run);
    days.insert(3, day03::run);
    days.insert(4, day04::run);
    days.insert(5, day05::run);
    days.insert(6, day06::run);
    days.insert(7, day07::run);
    days.insert(8, day08::run);
    days.insert(9, day09::run);
    days.insert(10, day10::run);
    days.insert(11, day11::run);
    days.insert(12, day12::run);
    days.insert(13, day13::run);
    days.insert(14, day14::run);
    days.insert(15, day15::run);


    if let Some(f) = days.get(&_day) {
        let now = Instant::now();
        f();
        println!("Elapsed: {}", now.elapsed().as_millis());
    };

    // /* Run all the days in order */
    // days.iter().sorted().for_each(|e| {
    //     println!("Day {}", e.0);
    //     let now = Instant::now();
    //     e.1();
    //     println!("Elapsed: {}", now.elapsed().as_millis());
    //     println!();
    // })
}
