use std::iter;
use std::str::FromStr;

use crate::utils::{date::Date, io::read_input};

#[derive(Clone, Copy)]
enum Operation {
    Square,
    Multiply(u32),
    Add(u32),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "new = old * old" {
            Ok(Operation::Square)
        } else if let Some(value) = s.strip_prefix("new = old * ") {
            value
                .parse::<u32>()
                .map(Operation::Multiply)
                .map_err(|_| ())
        } else if let Some(value) = s.strip_prefix("new = old + ") {
            value.parse::<u32>().map(Operation::Add).map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl Operation {
    fn apply(self, other: u64) -> u64 {
        match self {
            Self::Square => other * other,
            Self::Multiply(value) => other * value as u64,
            Self::Add(value) => other + value as u64,
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u32,
    if_true: usize,
    if_false: usize,
}

fn parse<'a, I, S>(lines: I) -> Vec<Monkey>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut iter = lines.into_iter().map(|line| line.as_ref());

    iter::from_fn(|| {
        iter.find(|line| !line.is_empty())?;

        let items = iter
            .next()?
            .rsplit_once(':')?
            .1
            .split(',')
            .map(|word| word.trim().parse().ok())
            .collect::<Option<_>>()?;

        let operation = iter.next()?.rsplit_once(": ")?.1.parse().ok()?;

        let test = iter
            .next()?
            .split_ascii_whitespace()
            .rev()
            .next()?
            .parse()
            .ok()?;

        let if_true = iter
            .next()?
            .split_ascii_whitespace()
            .rev()
            .next()?
            .parse()
            .ok()?;

        let if_false = iter
            .next()?
            .split_ascii_whitespace()
            .rev()
            .next()?
            .parse()
            .ok()?;

        Some(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
        })
    })
    .collect()
}

fn solve_puzzle<F: FnMut(u64) -> u64>(
    monkeys: &mut [Monkey],
    number_of_rounds: usize,
    mut calculate_relief: F,
) -> u64 {
    let mut counts = vec![0; monkeys.len()];

    for _ in 0..number_of_rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let items = monkey.items.drain(..).collect::<Vec<_>>();

            counts[i] += items.len();

            let Monkey {
                operation,
                test,
                if_true,
                if_false,
                ..
            } = *monkey;

            for item in items {
                let item = calculate_relief(operation.apply(item));

                let monkey_index = if item % test as u64 == 0 {
                    if_true
                } else {
                    if_false
                };

                monkeys[monkey_index].items.push(item);
            }
        }
    }

    counts.sort();

    counts
        .iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * (*x as u64))
}

fn part1<'a, I, S>(lines: I) -> u64
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    solve_puzzle(&mut parse(lines), 20, |x| x / 3)
}

fn part2<'a, I, S>(lines: I) -> u64
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut monkeys = parse(lines);

    let base = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test as u64);

    solve_puzzle(&mut monkeys, 10000, |x| x % base)
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day11).expect("Failed to read input");

    println!("Day 1 Part 1: {:?}", part1(&input));
    println!("Day 1 Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    fn get_input() -> Vec<String> {
        INPUT
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn day11_part1() {
        let actual = get_input();
        assert_eq!(part1(&actual), 10605);
    }

    #[test]
    fn day11_part2() {
        let actual = get_input();
        assert_eq!(part2(&actual), 2713310158);
    }
}
