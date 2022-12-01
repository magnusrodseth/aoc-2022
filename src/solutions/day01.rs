use crate::utils::{date::Date, io::read_input};

fn get_calorie_sums(input: &Vec<String>) -> Vec<i32> {
    let parsed: Vec<Vec<i32>> = input
        .split(|item| item.is_empty())
        .map(|group| group.iter().map(|s| s.parse().unwrap()).collect())
        .collect();

    parsed.iter().map(|group| group.iter().sum()).collect()
}

fn part1(input: &Vec<String>) -> i32 {
    let calorie_sums = get_calorie_sums(input);
    let max_calories = calorie_sums.iter().max().unwrap().to_owned();

    max_calories
}

fn part2(input: &Vec<String>) -> i32 {
    let mut calorie_sums = get_calorie_sums(input);

    // Sort in descending order
    calorie_sums.sort_by(|a, b| b.cmp(a));

    let top_three = calorie_sums.iter().take(3).copied().collect::<Vec<i32>>();

    top_three.iter().sum()
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day01).expect("Failed to read input");

    println!("Day 1 Part 1: {:?}", part1(&input));
    println!("Day 1 Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = read_input(Date::Day01).expect("Failed to read input");
        assert_eq!(part1(&input), 67450);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = read_input(Date::Day01).expect("Failed to read input");
        assert_eq!(part2(&input), 199357);
    }
}
