use aoc_2022::{solve, utils::date::Date};
use clap::Parser;

/// Advent of Code 2022
#[derive(Parser)]
struct Args {
    /// The date of the month
    #[arg(value_enum)]
    date: Date,
}

fn main() {
    let args = Args::parse();
    solve(args.date);
}
