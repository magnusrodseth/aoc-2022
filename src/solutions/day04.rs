use crate::utils::{date::Date, io::read_input};

enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Interval {
    min: u32,
    max: u32,
}

impl Interval {
    fn includes(&self, other: &Interval) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

fn pairs(input: &[String]) -> Vec<Vec<Interval>> {
    input
        .iter()
        .map(|line| {
            let pairs = line
                .split(',')
                .map(|pair| {
                    let interval = pair
                        .split('-')
                        .map(|s| s.parse::<u32>().expect("Failed to parse number"))
                        .collect::<Vec<_>>();

                    let min = interval[0];
                    let max = interval[1];

                    Interval { min, max }
                })
                .collect::<Vec<_>>();

            pairs
        })
        .collect::<Vec<_>>()
}

fn overlapping(pairs: &[Vec<Interval>], part: Part) -> i32 {
    pairs
        .iter()
        .filter(|pairs| {
            let first = &pairs[0];
            let second = &pairs[1];

            match part {
                Part::Part1 => first.includes(second) || second.includes(first),
                Part::Part2 => first.overlaps(second) || second.overlaps(first),
            }
        })
        .count() as i32
}

fn part1(input: &[String]) -> i32 {
    let pairs = pairs(input);
    overlapping(&pairs, Part::Part1)
}

fn part2(input: &[String]) -> i32 {
    let pairs = pairs(input);
    overlapping(&pairs, Part::Part2)
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day04)
        .expect("failed to read input")
        .into_iter()
        .filter(|line: &String| !line.is_empty())
        .collect();

    println!("Day 1 Part 1: {:?}", part1(&input));
    println!("Day 1 Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    fn get_input() -> Vec<String> {
        INPUT
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn day04_part1() {
        let actual = get_input();
        assert_eq!(part1(&actual), 2);
    }

    #[test]
    fn day04_part2() {
        let actual = get_input();
        assert_eq!(part2(&actual), 4);
    }
}
