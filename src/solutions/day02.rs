use crate::utils::{date::Date, io::read_input};

enum Part {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn opponent_choice(choice: char) -> Self {
        match choice {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            _ => panic!("invalid opponent choice"),
        }
    }

    /// In part 1, a `char` simply corresponds to a choice.
    fn part1_player_choice(choice: char) -> Self {
        match choice {
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => panic!("invalid player choice"),
        }
    }

    /// In part 2, the `char` choice is a bit more complex.
    /// Based on the opponent's choice, the `char` choice describes whether
    /// the player should lose, draw or win against the opponent.
    fn part2_player_choice(opponent: &Choice, player_choice: char) -> Self {
        match opponent {
            Choice::Rock => match player_choice {
                'X' => Choice::Scissors,
                'Y' => Choice::Rock,
                'Z' => Choice::Paper,
                _ => panic!("invalid player choice"),
            },
            Choice::Paper => match player_choice {
                'X' => Choice::Rock,
                'Y' => Choice::Paper,
                'Z' => Choice::Scissors,
                _ => panic!("invalid player choice"),
            },
            Choice::Scissors => match player_choice {
                'X' => Choice::Paper,
                'Y' => Choice::Scissors,
                'Z' => Choice::Rock,
                _ => panic!("invalid player choice"),
            },
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    player: Choice,
}

impl Round {
    fn outcome(opponent: &Choice, player: &Choice) -> i32 {
        match opponent {
            Choice::Rock => match player {
                Choice::Rock => 3,
                Choice::Paper => 6,
                Choice::Scissors => 0,
            },
            Choice::Paper => match player {
                Choice::Rock => 0,
                Choice::Paper => 3,
                Choice::Scissors => 6,
            },
            Choice::Scissors => match player {
                Choice::Rock => 6,
                Choice::Paper => 0,
                Choice::Scissors => 3,
            },
        }
    }
}

fn total_score(rounds: &[Round]) -> i32 {
    rounds
        .iter()
        .map(|round| {
            let score = round.player.score();
            let outcome = Round::outcome(&round.opponent, &round.player);

            score + outcome
        })
        .sum()
}

fn get_rounds(input: &[String], part: Part) -> Vec<Round> {
    input
        .iter()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|choices| {
            let opponent_choice = choices[0].parse::<char>().expect("invalid opponent choice");
            let player_choice = choices[1].parse::<char>().expect("invalid player choice");

            let opponent = Choice::opponent_choice(opponent_choice);

            let player = match part {
                Part::Part1 => Choice::part1_player_choice(player_choice),
                Part::Part2 => Choice::part2_player_choice(&opponent, player_choice),
            };

            Round { opponent, player }
        })
        .collect::<Vec<Round>>()
}

fn part1(input: &[String]) -> i32 {
    let rounds = get_rounds(input, Part::Part1);
    total_score(&rounds)
}

fn part2(input: &[String]) -> i32 {
    let rounds = get_rounds(input, Part::Part2);
    total_score(&rounds)
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day02)
        .expect("failed to read input")
        .into_iter()
        .filter(|line: &String| !line.is_empty())
        .collect();

    println!("Day 1 Part 1: {:?}", part1(&input));
    println!("Day 1 Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    fn get_input() -> Vec<String> {
        INPUT
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn test_part1() {
        let actual = get_input();
        assert_eq!(part1(&actual), 15);
    }

    #[test]
    fn test_part2() {
        let actual = get_input();
        assert_eq!(part2(&actual), 12);
    }
}
