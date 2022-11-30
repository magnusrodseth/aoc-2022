use crate::utils::{date::Date, io::read_input};

fn part1(input: &[i32]) -> i32 {
    0
}

fn part2(input: &[i32]) -> i32 {
    0
}

pub fn solve() -> i32 {
    let input: Vec<i32> = read_input(Date::Day01).expect("Failed to read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[0]), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[0]), 0);
    }
}
