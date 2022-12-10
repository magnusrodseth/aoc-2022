use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::utils::{date::Date, io::read_input};

#[derive(Clone, Copy)]
enum CpuState {
    Idle,
    Busy(Instruction, usize),
}

impl CpuState {
    /// Returns `true` if the CPU state is [`Idle`].
    ///
    /// [`Idle`]: CPUState::Idle
    #[must_use]
    fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }
}

struct Cpu {
    register_value: i64,
    state: CpuState,
    cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            register_value: 1,
            state: CpuState::Idle,
            cycle: 1,
        }
    }

    fn load_instruction(&mut self, instruction: Instruction) -> anyhow::Result<()> {
        self.state
            .is_idle()
            .then_some(())
            .ok_or_else(|| anyhow!("CPU is already running"))
            .map(|_| {
                self.state = CpuState::Busy(instruction, self.cycle);
            })
    }

    fn add(&mut self, value: i64) {
        self.register_value += value;
    }

    fn update(&mut self) {
        if let CpuState::Busy(instruction, cycle) = self.state {
            if self.cycle - cycle == instruction.cycle_length() {
                self.state = CpuState::Idle;

                if let Instruction::Add(value) = instruction {
                    self.add(value);
                }
            }
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        self.update();
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Noop,
    Add(i64),
}

impl Instruction {
    fn cycle_length(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Add(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let instruction = words.next().expect("No instruction");
        let argument = words.next();

        let argument = match argument {
            Some(arg) => arg.parse::<i64>()?,
            None => 0,
        };

        let instruction = match instruction {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Add(argument),
            _ => unreachable!("Unknown instruction"),
        };

        Ok(instruction)
    }
}

fn part1(input: &[String]) -> i64 {
    let checkpoints = (20..=220).step_by(40).collect::<Vec<_>>();

    let mut instructions = input.iter().map(|line| {
        line.parse::<Instruction>()
            .expect("Failed to parse instruction")
    });
    let mut signal_strength = 0;
    let mut cpu = Cpu::new();

    loop {
        if cpu.state.is_idle() {
            if let Some(instruction) = instructions.next() {
                cpu.load_instruction(instruction)
                    .expect("Failed to load instruction");
            }
        }

        cpu.tick();

        if checkpoints.contains(&cpu.cycle) {
            signal_strength += (cpu.cycle as i64) * cpu.register_value;
        }

        let is_past_last_checkpoint = cpu.cycle == checkpoints.last().expect("No checkpoints") + 1;

        if is_past_last_checkpoint {
            break;
        }
    }

    signal_strength
}

fn part2(input: &[String]) {
    const RIGHT_MOST_PIXEL_IN_ROW: i64 = 39;
    const LEFT_MOST_PIXEL_IN_ROW: i64 = 0;

    let checkpoints = (20..=220).step_by(40).collect::<Vec<_>>();
    let mut instructions = input.iter().map(|line| {
        line.parse::<Instruction>()
            .expect("Failed to parse instruction")
    });
    let mut cpu = Cpu::new();
    let mut crt_position = LEFT_MOST_PIXEL_IN_ROW;

    loop {
        if cpu.state.is_idle() {
            if let Some(instruction) = instructions.next() {
                cpu.load_instruction(instruction)
                    .expect("Failed to load instruction");
            }
        }

        let three_pixel_range = cpu.register_value - 1..=cpu.register_value + 1;

        if three_pixel_range.contains(&crt_position) {
            print!("█")
        } else {
            print!(" ")
        }

        cpu.tick();
        crt_position += 1;

        if crt_position > RIGHT_MOST_PIXEL_IN_ROW {
            crt_position = LEFT_MOST_PIXEL_IN_ROW;
            println!();
        }

        let is_past_last_checkpoint = cpu.cycle == checkpoints.last().expect("No checkpoints") + 1;
        if is_past_last_checkpoint {
            break;
        }
    }
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day10).expect("failed to read input");

    println!("Day 1 Part 1: {:?}", part1(&input));

    println!("Day 1 Part 2:");
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    fn get_input() -> Vec<String> {
        INPUT
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn day09_part1() {
        let actual = get_input();
        assert_eq!(part1(&actual), 13140);
    }
}
