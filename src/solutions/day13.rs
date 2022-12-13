use std::{cmp::Ordering, iter::Peekable};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Packet(Vec<Item>);

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        slice_cmp(&self.0, &other.0)
    }
}

fn slice_cmp(first: &[Item], second: &[Item]) -> Ordering {
    for (a, b) in first.iter().zip(second.iter()) {
        match a.cmp(b) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Greater,
        }
    }

    first.len().cmp(&second.len())
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Item {
    Value(usize),
    Packet(Packet),
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Value(first_value), Item::Value(second_value)) => first_value.cmp(second_value),
            (Item::Value(first_value), Item::Packet(second_packet)) => {
                slice_cmp(&[Item::Value(*first_value)], &second_packet.0)
            }
            (Item::Packet(_), Item::Value(_)) => other.cmp(self).reverse(),
            (Item::Packet(first_packet), Item::Packet(second_packet)) => {
                first_packet.cmp(second_packet)
            }
        }
    }
}

fn parse_item<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> Option<Item> {
    let first = *input.peek().expect("no first item");

    if ('0'..='9').contains(&first) {
        input.next();
        let mut item = first as usize - '0' as usize;

        if item == 1 {
            let next = *input.peek().expect("no next item");

            if next == '0' {
                item = 10;
                input.next();
            }
        }

        Some(Item::Value(item as usize))
    } else if first == ']' {
        None
    } else {
        Some(Item::Packet(parse_packet(input)))
    }
}

fn parse_packet<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> Packet {
    let mut items = Vec::new();

    while let Some(value) = input.next() {
        match value {
            ',' | '[' => {
                if let Some(item) = parse_item(input) {
                    items.push(item);
                }
            }
            ']' => return Packet(items),
            _ => {
                panic!("unexpected item: {}", value);
            }
        }
    }

    Packet(items)
}

fn parsed(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|line| {
            let (first, second) = line.split_once('\n').expect("Invalid input");
            let (mut first, mut second) = (first.chars().peekable(), second.chars().peekable());

            (parse_packet(&mut first), parse_packet(&mut second))
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let parsed = parsed(input);

    let pairs_in_correct_order = parsed
        .iter()
        .enumerate()
        .filter(|(_, (first, second))| first.cmp(second) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    pairs_in_correct_order
}

fn part2(input: &str) -> usize {
    let parsed = parsed(input);

    let first_divider = Packet(vec![Item::Packet(Packet(vec![Item::Value(2)]))]);
    let second_divider = Packet(vec![Item::Packet(Packet(vec![Item::Value(6)]))]);

    let mut all_packets = parsed
        .into_iter()
        .flat_map(|(first, second)| [first, second].into_iter())
        .collect::<Vec<_>>();
    all_packets.push(first_divider.clone());
    all_packets.push(second_divider.clone());

    all_packets.sort();

    let first_divider_index = all_packets
        .iter()
        .position(|packet| packet == &first_divider)
        .expect("first divider not found");

    let second_divider_index = all_packets
        .iter()
        .position(|packet| packet == &second_divider)
        .expect("second divider not found");

    // Decoder key
    (first_divider_index + 1) * (second_divider_index + 1)
}

pub fn solve() {
    let input = include_str!("../../input/day13.txt");

    println!("Day 1 Part 1: {:?}", part1(input));
    println!("Day 1 Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn day13_part1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn day13_part2() {
        assert_eq!(part2(INPUT), 140);
    }
}
