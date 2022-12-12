use pathfinding::prelude::{bfs, Matrix};

struct Cell {
    row: usize,
    col: usize,
}

impl From<(usize, usize)> for Cell {
    fn from((row, col): (usize, usize)) -> Self {
        Cell { row, col }
    }
}

struct Grid {
    matrix: Matrix<u8>,
    start: Cell,
    end: Cell,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let mut matrix = Matrix::from_rows(input.lines().map(str::bytes)).expect("Invalid input");

        let start = Cell::from(
            matrix
                .indices()
                .find(|c| matrix[*c] == b'S')
                .expect("No start"),
        );

        let end = Cell::from(
            matrix
                .indices()
                .find(|c| matrix[*c] == b'E')
                .expect("No end"),
        );

        matrix[(start.row, start.col)] = b'a';
        matrix[(end.row, end.col)] = b'z';

        Grid { matrix, start, end }
    }
}

fn part1(input: &str) -> usize {
    let grid = &Grid::from_input(input);

    bfs(
        &(grid.start.row, grid.start.col),
        |&coord| {
            grid.matrix
                .neighbours(coord, false)
                .filter(move |&other_coord| grid.matrix[other_coord] <= grid.matrix[coord] + 1)
        },
        |&p| p == (grid.end.row, grid.end.col),
    )
    .expect("An error occurred when performing BFS")
    .len()
        - 1
}

fn part2(input: &str) -> usize {
    let grid = &Grid::from_input(input);

    bfs(
        &(grid.end.row, grid.end.col),
        |&coord| {
            grid.matrix
                .neighbours(coord, false)
                .filter(move |&other_coord| grid.matrix[coord] <= grid.matrix[other_coord] + 1)
        },
        |&coord| grid.matrix[coord] == b'a',
    )
    .unwrap()
    .len()
        - 1
}

pub fn solve() {
    let input = include_str!("../../input/day12.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn day12_part1() {
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn day12_part2() {
        assert_eq!(part2(INPUT), 29);
    }
}
