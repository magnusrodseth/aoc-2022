use std::array::from_fn;

type Stacks<const STACKS_COUNT: usize> = [Vec<char>; STACKS_COUNT];

struct Instruction {
    quantity: u8,
    from: u8,
    to: u8,
}

fn parse_input<const STACKS_COUNT: usize>(input: &str) -> (Stacks<STACKS_COUNT>, Vec<Instruction>) {
    let (crates_input, instruction_input) = input.split_once("\n\n").expect("Invalid input");

    // Infer the type from the function signature
    let mut crates = from_fn(|_| Vec::new());

    for line in crates_input.lines().rev() {
        let characters = line.chars().skip(1).step_by(4);

        for (i, character) in characters.enumerate() {
            if let character @ 'A'..='Z' = character {
                crates[i].push(character)
            }
        }
    }

    let mut instruction = instruction_input
        .split_ascii_whitespace()
        .skip(1)
        .step_by(2)
        .map(|num| num.parse().expect("Failed to parse number"));
    let mut instructions = Vec::new();

    while let (Some(quantity), Some(from), Some(to)) =
        (instruction.next(), instruction.next(), instruction.next())
    {
        instructions.push(Instruction { quantity, from, to });
    }

    (crates, instructions)
}

fn part1<const NUMBER_OF_STACKS: usize>(
    (mut crates, instructions): (Stacks<NUMBER_OF_STACKS>, &Vec<Instruction>),
) -> String {
    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let value = crates[instruction.from as usize - 1]
                .pop()
                .expect("No crates left");

            crates[instruction.to as usize - 1].push(value);
        }
    }

    let top_crates = crates
        .iter_mut()
        .map(|stack| stack.pop().expect("No crates left"))
        .collect();

    top_crates
}

fn part2<const NUMBER_OF_STACKS: usize>(
    (mut crates, instructions): (Stacks<NUMBER_OF_STACKS>, &Vec<Instruction>),
) -> String {
    for instruction in instructions {
        let (quantity, from, to) = (
            instruction.quantity as usize,
            instruction.from as usize - 1,
            instruction.to as usize - 1,
        );

        let mut crates_to_move = crates[from].split_off(crates[from].len() - quantity);
        crates[to].append(&mut crates_to_move);
    }

    crates
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}

pub fn solve() {
    let input = include_str!("../../input/day05.txt");
    let (crates, instructions) = parse_input::<9>(input);

    // println!("Day 1 Part 1: {:?}", part1::<9>((crates, &instructions)));
    println!("Day 1 Part 2: {:?}", part2::<9>((crates, &instructions)));
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
        assert_eq!(part1((crates, &instructions)), "CMZ");
    }

    #[test]
    fn day05_part2() {
        let (crates, instructions) = parse_input::<3>(INPUT);
        assert_eq!(part2((crates, &instructions)), "MCD");
    }
}
