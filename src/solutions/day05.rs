use std::array::from_fn;

#[derive(Clone)]
struct Instruction {
    quantity: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn parse(input: &str) -> Vec<Self> {
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
}

impl<const STACKS_COUNT: usize> Crates<STACKS_COUNT> {
    /// Creates new crates from a string of characters.
    /// The string is split into `STACKS_COUNT` stacks.
    fn parse(input: &str) -> Self {
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

    /// Pops off the top crate from each stack in the collection of crates.
    fn top_crates(&mut self) -> String {
        self.stacks
            .iter_mut()
            .map(|stack| stack.pop().expect("No crates left"))
            .collect()
    }

    fn move_one(&mut self, from: usize, to: usize) {
        let value = self.stacks[from].pop().expect("No crates left");
        self.stacks[to].push(value);
    }

    fn move_many(&mut self, from: usize, to: usize, quantity: usize) {
        let mut crates_to_move = self.stacks[from].split_off(self.stacks[from].len() - quantity);
        self.stacks[to].append(&mut crates_to_move);
    }
}

struct CrateContainer<const STACKS_COUNT: usize> {
    crates: Crates<STACKS_COUNT>,
    instructions: Vec<Instruction>,
}

fn parse_input<const STACKS_COUNT: usize>(input: &str) -> CrateContainer<STACKS_COUNT> {
    let (crates_input, instruction_input) = input.split_once("\n\n").expect("Invalid input");
    let instructions = Instruction::parse(instruction_input);
    let crates = Crates::parse(crates_input);

    CrateContainer {
        crates,
        instructions,
    }
}

fn part1<const STACKS_COUNT: usize>(container: &CrateContainer<STACKS_COUNT>) -> String {
    // We clone the borrowed `crates` to avoid borrowing it mutably.
    // This way, part1 and part2 can be called on the same crates.
    let mut crates = container.crates.clone();
    let instructions = &container.instructions;

    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let from = instruction.from as usize - 1;
            let to = instruction.to as usize - 1;

            crates.move_one(from, to);
        }
    }

    crates.top_crates()
}

fn part2<const STACKS_COUNT: usize>(container: &CrateContainer<STACKS_COUNT>) -> String {
    // We clone the borrowed `crates` to avoid borrowing it mutably.
    // This way, part1 and part2 can be called on the same crates.
    let mut crates = container.crates.clone();
    let instructions = &container.instructions;

    for instruction in instructions {
        let (quantity, from, to) = (
            instruction.quantity as usize,
            instruction.from as usize - 1,
            instruction.to as usize - 1,
        );

        crates.move_many(from, to, quantity);
    }

    crates.top_crates()
}

pub fn solve() {
    let input = include_str!("../../input/day05.txt");
    let container = parse_input::<9>(input);

    println!("Day 1 Part 1: {:?}", part1::<9>(&container));
    println!("Day 1 Part 2: {:?}", part2::<9>(&container));
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
        let container = parse_input::<3>(INPUT);
        assert_eq!(part1(&container), "CMZ");
    }

    #[test]
    fn day05_part2() {
        let container = parse_input::<3>(INPUT);
        assert_eq!(part2(&container), "MCD");
    }
}
