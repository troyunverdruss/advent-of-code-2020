use std::collections::HashMap;

mod d2019_1;
mod util;
mod day01;
mod day02;


fn main() {
    println!("Hello, world!");

    let day = 2;
    //
    // let mut days = HashMap::new();
    // days.insert(20191, d2019_1::run);
    // days.insert(1, day01::run);

    match day {
        20191 => d2019_1::run(),
        1 => day01::run(),
        2 => day02::run(),
        _ => panic!("Unknown day"),
    }
}
