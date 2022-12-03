use itertools::Itertools;

use crate::utils::{date::Date, io::read_input};

/// Closure that calculates the priority of a character.
/// Characters a - z have a priority of 1 - 26.
/// Characters A - Z have a priority of 27 - 52.
/// Characters that are not letters have a priority of 0.
fn calculate_priority() -> impl Fn(Option<char>) -> i32 {
    |item_type| {
        if let Some(character) = item_type {
            match character {
                'a'..='z' => (character as i32) - ('a' as i32) + 1,
                'A'..='Z' => (character as i32) - ('A' as i32) + 27,
                _ => 0,
            }
        } else {
            0
        }
    }
}

fn part1(input: &[String]) -> i32 {
    let priority_sum = input
        .iter()
        .map(|line| {
            // Get the common character in the first half and the second half of the line.
            let middle = line.len() / 2;
            let first_half = &line[..middle];
            let second_half = &line[middle..];

            let mut common: Option<char> = None;
            for first_char in first_half.chars() {
                for second_char in second_half.chars() {
                    if first_char == second_char {
                        common = Some(first_char);
                    }
                }
            }

            common
        })
        .map(calculate_priority())
        .sum::<i32>();

    priority_sum
}

fn part2(input: &[String]) -> i32 {
    let priority_sum = input
        .iter()
        .tuples::<(_, _, _)>()
        .map(|(first, second, third)| {
            // Iterate over all possible combinations of the three lines
            let mut common: Option<char> = None;
            for first_char in first.chars() {
                for second_char in second.chars() {
                    for third_char in third.chars() {
                        if first_char == second_char && second_char == third_char {
                            common = Some(first_char);
                        }
                    }
                }
            }

            common
        })
        .map(calculate_priority())
        .sum::<i32>();

    priority_sum
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day03).expect("failed to read input");

    println!("Day 1 Part 1: {:?}", part1(&input));
    println!("Day 1 Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    fn get_input() -> Vec<String> {
        INPUT
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn day03_part1() {
        let actual = get_input();
        assert_eq!(part1(&actual), 157);
    }

    #[test]
    fn day03_part2() {
        let actual = get_input();
        assert_eq!(part2(&actual), 70);
    }
}
