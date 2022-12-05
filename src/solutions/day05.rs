use std::array::from_fn;

#[derive(Clone)]
struct Instruction {
    quantity: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn parse_instructions(input: &str) -> Vec<Self> {
        let mut instruction = input
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|num| num.parse().expect("Failed to parse number"));

        let mut instructions = Vec::new();

        while let (Some(quantity), Some(from), Some(to)) =
            (instruction.next(), instruction.next(), instruction.next())
        {
            instructions.push(Instruction { quantity, from, to });
        }

        instructions
    }
}

#[derive(Clone)]
struct Crates<const STACKS_COUNT: usize> {
    stacks: [Vec<char>; STACKS_COUNT],
    instructions: Vec<Instruction>,
}

impl<const STACKS_COUNT: usize> Crates<STACKS_COUNT> {
    /// Creates new crates from a string of characters.
    /// The string is split into `STACKS_COUNT` stacks.
    fn new(input: &str, instructions: Vec<Instruction>) -> Self {
        let mut stacks = from_fn(|_| Vec::new());

        for line in input.lines().rev() {
            let characters = line.chars().skip(1).step_by(4);

            for (i, character) in characters.enumerate() {
                if let character @ 'A'..='Z' = character {
                    stacks[i].push(character)
                }
            }
        }

        Self {
            stacks,
            instructions,
        }
    }

    fn top_crates(&mut self) -> String {
        self.stacks
            .iter_mut()
            .map(|stack| stack.pop().expect("No crates left"))
            .collect()
    }
}

/// Parsing input is the biggest hassle in this solution.
/// First, we must separate the stacks from the instructions.
fn parse_input<const STACKS_COUNT: usize>(input: &str) -> Crates<STACKS_COUNT> {
    let (crates_input, instruction_input) = input.split_once("\n\n").expect("Invalid input");
    let instructions = Instruction::parse_instructions(instruction_input);

    Crates::new(crates_input, instructions)
}

fn part1<const STACKS_COUNT: usize>(crates: &Crates<STACKS_COUNT>) -> String {
    // We clone the borrowed `crates` to avoid borrowing it mutably.
    // This way, part1 and part2 can be called on the same crates.
    let mut crates = crates.clone();

    for instruction in &crates.instructions {
        for _ in 0..instruction.quantity {
            let value = crates.stacks[instruction.from as usize - 1]
                .pop()
                .expect("No crates left");

            crates.stacks[instruction.to as usize - 1].push(value);
        }
    }

    crates.top_crates()
}

fn part2<const STACKS_COUNT: usize>(crates: &Crates<STACKS_COUNT>) -> String {
    // We clone the borrowed `crates` to avoid borrowing it mutably.
    // This way, part1 and part2 can be called on the same crates.
    let mut crates = crates.clone();

    for instruction in &crates.instructions {
        let (quantity, from, to) = (
            instruction.quantity as usize,
            instruction.from as usize - 1,
            instruction.to as usize - 1,
        );

        let mut crates_to_move =
            crates.stacks[from].split_off(crates.stacks[from].len() - quantity);
        crates.stacks[to].append(&mut crates_to_move);
    }

    crates.top_crates()
}

pub fn solve() {
    let input = include_str!("../../input/day05.txt");
    let crates = parse_input::<9>(input);

    println!("Day 1 Part 1: {:?}", part1::<9>(&crates));
    println!("Day 1 Part 2: {:?}", part2::<9>(&crates));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn day05_part1() {
        let crates = parse_input::<3>(INPUT);
        assert_eq!(part1(&crates), "CMZ");
    }

    #[test]
    fn day05_part2() {
        let crates = parse_input::<3>(INPUT);
        assert_eq!(part2(&crates), "MCD");
    }
}
