# 🎅 🦀 Advent of Code 2022

## Developer Information

Developed by Magnus Rødseth.

## Running the application

```sh
# Navigate to the project directory
cd aoc-2022

# View available command line arguments
cargo run -- --help

# Run the solution for a given date (day01 - day25)
cargo run <DATE>

# Run tests with sample input for a given date (day01 - day25)
cargo test <DATE>
```

## Solving tasks

Each file in `src/solutions` corresponds to one day of puzzles.

1. For each day, simply copy `src/solutions/day01.rs` and rename it to the correct day. 
2. Next, add it as a module in `src/solutions/mod.rs`.
3. Lastly, call the correct day's `solve` function in `lib.rs`.
