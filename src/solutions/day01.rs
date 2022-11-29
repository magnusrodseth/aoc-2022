use crate::utils::{date::Date, io::read_input};

pub fn solve() -> i32 {
    let input: Vec<i32> = read_input(Date::Day01).expect("Failed to read input");

    println!("Day 01 {:?}", input);

    0
}
