use std::{collections::HashSet, str::FromStr};

use crate::utils::{date::Date, io::read_input};

enum Movement {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    movement: Movement,
    times: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (movement, distance) = s.split_once(' ').expect("Invalid input");

        let movement = match movement {
            "R" => Movement::Right,
            "L" => Movement::Left,
            "U" => Movement::Up,
            "D" => Movement::Down,
            _ => unreachable!("Invalid input"),
        };

        let distance = distance.parse::<usize>().expect("Invalid value");

        Ok(Self {
            movement,
            times: distance,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn is_adjacent_or_overlaps(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

#[derive(Clone)]
struct Knot {
    position: Coordinate,
    visited: HashSet<Coordinate>,
}

impl Knot {
    fn new(position: Coordinate) -> Self {
        let visited = HashSet::from([position]);
        Self { position, visited }
    }

    fn move_to(&mut self, position: Coordinate) {
        self.position = position;
        self.visited.insert(position);
    }
}

struct KnottedRope<const KNOTS_COUNT: usize> {
    knots: Vec<Knot>,
}

impl<const KNOTS_COUNT: usize> KnottedRope<KNOTS_COUNT> {
    fn new() -> Self {
        let start = Coordinate { x: 0, y: 0 };
        let knots = vec![Knot::new(start); KNOTS_COUNT];
        Self { knots }
    }

    fn tail(&self) -> &Knot {
        self.knots.last().expect("No knots")
    }

    fn update(&mut self, instruction: &Instruction) {
        let mut iterations = instruction.times;

        while iterations > 0 {
            let mut previous_position: Option<Coordinate> = None;

            for knot in self.knots.iter_mut() {
                let Coordinate { x, y } = knot.position;

                match previous_position {
                    Some(previous_position) => {
                        if !knot.position.is_adjacent_or_overlaps(&previous_position) {
                            let Coordinate {
                                x: previous_x,
                                y: previous_y,
                            } = previous_position;

                            if x == previous_x {
                                if y < previous_y {
                                    knot.move_to(Coordinate { x, y: y + 1 });
                                } else {
                                    knot.move_to(Coordinate { x, y: y - 1 });
                                }
                            } else if y == previous_y {
                                if x < previous_x {
                                    knot.move_to(Coordinate { x: x + 1, y });
                                } else {
                                    knot.move_to(Coordinate { x: x - 1, y });
                                }
                            } else if x < previous_x {
                                if y < previous_y {
                                    knot.move_to(Coordinate { x: x + 1, y: y + 1 });
                                } else {
                                    knot.move_to(Coordinate { x: x + 1, y: y - 1 });
                                }
                            } else if y < previous_y {
                                knot.move_to(Coordinate { x: x - 1, y: y + 1 });
                            } else {
                                knot.move_to(Coordinate { x: x - 1, y: y - 1 });
                            }
                        }
                    }
                    None => {
                        match instruction.movement {
                            Movement::Up => {
                                knot.move_to(Coordinate { x, y: y + 1 });
                            }
                            Movement::Down => {
                                knot.move_to(Coordinate { x, y: y - 1 });
                            }
                            Movement::Left => {
                                knot.move_to(Coordinate { x: x - 1, y });
                            }
                            Movement::Right => {
                                knot.move_to(Coordinate { x: x + 1, y });
                            }
                        };
                    }
                }

                previous_position = Some(knot.position);
            }

            iterations -= 1;
        }
    }
}

fn visited_by_tail<const KNOTS_COUNT: usize>(input: &[String]) -> usize {
    let mut rope = KnottedRope::<KNOTS_COUNT>::new();

    let instructions = input
        .iter()
        .map(|line| line.parse::<Instruction>().expect("Invalid input"))
        .collect::<Vec<_>>();

    for instruction in instructions {
        rope.update(&instruction);
    }

    rope.tail().visited.len()
}

fn part1(input: &[String]) -> usize {
    visited_by_tail::<2>(input)
}

fn part2(input: &[String]) -> usize {
    visited_by_tail::<10>(input)
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day09).expect("failed to read input");

    println!("Day 1 Part 1: {:?}", part1(&input));
    println!("Day 1 Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const BIGGER_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    fn get_input(input: &str) -> Vec<String> {
        input
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn day09_part1() {
        let actual = get_input(BASE_INPUT);
        assert_eq!(part1(&actual), 13);
    }

    #[test]
    fn day09_part2() {
        let actual = get_input(BASE_INPUT);
        assert_eq!(part2(&actual), 1);
    }

    #[test]
    fn day09_part2_bigger() {
        let actual = get_input(BIGGER_INPUT);
        assert_eq!(part2(&actual), 36);
    }
}
