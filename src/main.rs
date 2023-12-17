mod cli;
mod year2023;
mod aoc;

use std::error::Error;
use clap::Parser;
use tokio::main;

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Arguments::parse();
    let session_token = aoc::session_token::from_env("AOC_SESSION_TOKEN")?;

    println!("Session token: {}", session_token.0);

    let puzzle_input = aoc::puzzle_input::fetch(&session_token.0, aoc::Year(args.year), aoc::Day(args.day)).await
        .map_err(|error| format!("Failed to fetch puzzle input for: {}", error))?;

    println!("Puzzle input:\n{}", puzzle_input.0);

    match args.year {
        2023 => {
            println!("Day {} for year {} not implemented", args.day, args.year);
        }
        _ => println!("Invalid year"),
    }

    println!("Year: {}", args.year);

    Ok(())
}
