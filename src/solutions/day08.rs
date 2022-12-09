use std::cmp;

use crate::utils::{date::Date, io::read_input};

struct Visible {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

#[derive(Debug)]
struct Forest {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<u32>>,
}

impl Forest {
    fn new(input: &[String]) -> Self {
        let grid = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|character| character.to_digit(10).expect("Invalid character"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = grid[0].len();
        let cols = grid.len();

        Self { rows, cols, grid }
    }

    /// Determines if the given tree in the forest is visible.
    /// A tree is visible if all other trees between it and any edge
    /// of the grid is shorter than it.
    fn is_visible(&self, row_index: usize, col_index: usize) -> bool {
        let current_value = self.grid[row_index][col_index];

        let is_visible = Visible {
            left: self.grid[row_index][0..col_index]
                .iter()
                .all(|&x| x < current_value),
            right: self.grid[row_index][col_index + 1..]
                .iter()
                .all(|&x| x < current_value),
            top: self.grid[0..row_index]
                .iter()
                .all(|row| row[col_index] < current_value),
            bottom: self.grid[row_index + 1..]
                .iter()
                .all(|row| row[col_index] < current_value),
        };

        is_visible.left || is_visible.right || is_visible.top || is_visible.bottom
    }

    /// Determines if a value is on the edge of the grid.
    /// A value is on the edge if it is on the first or last row or column.
    fn is_on_edge(&self, row_index: usize, col_index: usize) -> bool {
        row_index == 0 || row_index == self.cols - 1 || col_index == 0 || col_index == self.rows - 1
    }

    fn scenic_score(&self, row_index: usize, col_index: usize) -> usize {
        let current_value = self.grid[row_index][col_index];

        let mut score = 1;
        let mut run = 0;

        // Left
        for i in (0..col_index).rev() {
            run += 1;
            if self.grid[row_index][i] >= current_value {
                break;
            }
        }
        score *= run;

        // Right
        run = 0;
        for i in col_index + 1..self.rows {
            run += 1;
            if self.grid[row_index][i] >= current_value {
                break;
            }
        }
        score *= run;

        // Top
        run = 0;
        for i in (0..row_index).rev() {
            run += 1;
            if self.grid[i][col_index] >= current_value {
                break;
            }
        }
        score *= run;

        // Bottom
        run = 0;
        for i in row_index + 1..self.cols {
            run += 1;
            if self.grid[i][col_index] >= current_value {
                break;
            }
        }
        score *= run;

        score
    }
}

fn part1(input: &[String]) -> usize {
    let forest = Forest::new(input);

    forest
        .grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _)| forest.is_visible(i, *j) || forest.is_on_edge(i, *j))
                .count()
        })
        .sum::<usize>()
}

fn part2(input: &[String]) -> usize {
    let forest = Forest::new(input);

    let scores = forest
        .grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, _)| forest.scenic_score(i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_score = scores.iter().flatten().max().expect("No max score found");

    *max_score
}

pub fn solve() {
    let input: Vec<String> = read_input(Date::Day08).expect("failed to read input");

    println!("Day 1 Part 1: {:?}", part1(&input));
    println!("Day 1 Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    fn get_input() -> Vec<String> {
        INPUT
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn day08_part1() {
        let actual = get_input();
        assert_eq!(part1(&actual), 21);
    }

    #[test]
    fn day08_part2_demo() {
        let actual = get_input();
        let forest = Forest::new(&actual);
        assert_eq!(forest.scenic_score(1, 2), 4);
    }

    #[test]
    fn day08_part2() {
        let actual = get_input();
        assert_eq!(part2(&actual), 8);
    }
}
