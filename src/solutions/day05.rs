use std::array::from_fn;

type Stacks<const STACKS_COUNT: usize> = [Vec<char>; STACKS_COUNT];

struct Crates<const STACKS_COUNT: usize> {
    stacks: Stacks<STACKS_COUNT>,
}

impl<const STACKS_COUNT: usize> Crates<STACKS_COUNT> {
    /// Creates a new set of crates from a string of characters.
    /// The string is split into `STACKS_COUNT` stacks.
    fn new(input: &str) -> Self {
        let mut stacks = from_fn(|_| Vec::new());

        for line in input.lines().rev() {
            let characters = line.chars().skip(1).step_by(4);

            for (i, character) in characters.enumerate() {
                if let character @ 'A'..='Z' = character {
                    stacks[i].push(character)
                }
            }
        }

        Self { stacks }
    }
}

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

/// Parsing input is the biggest hassle in this solution.
/// First, we must separate the stacks from the instructions.
fn parse_input<const STACKS_COUNT: usize>(input: &str) -> (Crates<STACKS_COUNT>, Vec<Instruction>) {
    let (crates_input, instruction_input) = input.split_once("\n\n").expect("Invalid input");

    let mut crates = Crates::new(crates_input);
    let mut instructions = Instruction::parse_instructions(instruction_input);

    (crates, instructions)
}

fn part1<const STACKS_COUNT: usize>(
    mut crates: Crates<STACKS_COUNT>,
    instructions: &Vec<Instruction>,
) -> String {
    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let value = crates.stacks[instruction.from as usize - 1]
                .pop()
                .expect("No crates left");

            crates.stacks[instruction.to as usize - 1].push(value);
        }
    }

    let top_crates = crates
        .stacks
        .iter_mut()
        .map(|stack| stack.pop().expect("No crates left"))
        .collect();

    top_crates
}

fn part2<const STACKS_COUNT: usize>(
    mut crates: Crates<STACKS_COUNT>,
    instructions: &Vec<Instruction>,
) -> String {
    for instruction in instructions {
        let (quantity, from, to) = (
            instruction.quantity as usize,
            instruction.from as usize - 1,
            instruction.to as usize - 1,
        );

        let mut crates_to_move =
            crates.stacks[from].split_off(crates.stacks[from].len() - quantity);
        crates.stacks[to].append(&mut crates_to_move);
    }

    crates
        .stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}

pub fn solve() {
    let input = include_str!("../../input/day05.txt");
    let (crates, instructions) = parse_input::<9>(input);

    println!("Day 1 Part 1: {:?}", part1::<9>(crates, &instructions));
    // println!("Day 1 Part 2: {:?}", part2::<9>(crates, &instructions));
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
        let (crates, instructions) = parse_input::<3>(INPUT);
        assert_eq!(part1(crates, &instructions), "CMZ");
    }

    #[test]
    fn day05_part2() {
        let (crates, instructions) = parse_input::<3>(INPUT);
        assert_eq!(part2(crates, &instructions), "MCD");
    }
}
