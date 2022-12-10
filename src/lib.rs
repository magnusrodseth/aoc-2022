use crate::{
    solutions::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10},
    utils::date::Date,
};

pub mod solutions;
pub mod utils;

/// Wrapper for solving the problem for a given date.
pub fn solve(date: Date) {
    match date {
        Date::Day01 => day01::solve(),
        Date::Day02 => day02::solve(),
        Date::Day03 => day03::solve(),
        Date::Day04 => day04::solve(),
        Date::Day05 => day05::solve(),
        Date::Day06 => day06::solve(),
        Date::Day07 => day07::solve(),
        Date::Day08 => day08::solve(),
        Date::Day09 => day09::solve(),
        Date::Day10 => day10::solve(),
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
