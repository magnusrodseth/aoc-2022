use crate::solutions::day_01;

pub mod solutions;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Date {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

/// Wrapper for solving the problem for a given date
pub fn solve(date: Date) -> i32 {
    match date {
        Date::Day01 => day_01::solve(),
        Date::Day02 => todo!(),
        Date::Day03 => todo!(),
        Date::Day04 => todo!(),
        Date::Day05 => todo!(),
        Date::Day06 => todo!(),
        Date::Day07 => todo!(),
        Date::Day08 => todo!(),
        Date::Day09 => todo!(),
        Date::Day10 => todo!(),
        Date::Day11 => todo!(),
        Date::Day12 => todo!(),
        Date::Day13 => todo!(),
        Date::Day14 => todo!(),
        Date::Day15 => todo!(),
        Date::Day16 => todo!(),
        Date::Day17 => todo!(),
        Date::Day18 => todo!(),
        Date::Day19 => todo!(),
        Date::Day20 => todo!(),
        Date::Day21 => todo!(),
        Date::Day22 => todo!(),
        Date::Day23 => todo!(),
        Date::Day24 => todo!(),
        Date::Day25 => todo!(),
    }
}
