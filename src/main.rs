mod cli;
mod year2023;
mod aoc;

use std::error::Error;
use clap::Parser;
use tokio::main;

use crate::aoc::{RawPuzzleInput, Puzzle, puzzle_not_implemented, PuzzleResult, SessionToken};

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Arguments::parse();
    let session_token = SessionToken::from_env("AOC_SESSION_TOKEN")?;

    let raw_puzzle_input = RawPuzzleInput::fetch(&session_token.0, aoc::Year(args.year), aoc::Day(args.day)).await
        .map_err(|error| format!("Failed to fetch puzzle input for: {}", error))?;

    let puzzle_result: PuzzleResult = match args.year {
        2023 => {
            match args.day {
                1 => year2023::day1::Day1::solve(&raw_puzzle_input),
                _ => puzzle_not_implemented(),
            }
        }
        _ => puzzle_not_implemented()
    };

    println!("Advent of Code {} - Day {}", args.year, args.day);
    println!("{}", puzzle_result);

    Ok(())
}
