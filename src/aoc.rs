use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use reqwest::header::COOKIE;

#[derive(Debug)]
pub enum SessionTokenError {
    InvalidLength { expected: usize, actual: usize },
}

impl std::fmt::Display for SessionTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionTokenError::InvalidLength { expected, actual } => {
                write!(f, "invalid session token length, expected {}, got {}", expected, actual)
            }
        }
    }
}

impl Error for SessionTokenError {}

pub struct SessionToken(pub(crate) String);

impl FromStr for SessionToken {
    type Err = SessionTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPECTED_LENGTH: usize = 128;
        if s.len() != EXPECTED_LENGTH {
            return Err(SessionTokenError::InvalidLength {
                expected: EXPECTED_LENGTH,
                actual: s.len(),
            });
        }
        Ok(SessionToken(String::from(s)))
    }
}

impl SessionToken {
    pub fn from_env(key: &str) -> Result<SessionToken, Box<dyn Error>> {
        std::env::var(key)
            .map_err(|error| format!("Failed to read session token from environment variable {}: {}", key, error))?
            .parse::<SessionToken>()
            .map_err(|error| format!("Failed to parse session token: {}", error).into())
    }
}


pub struct Year(pub u16);

pub struct Day(pub u8);

pub struct RawPuzzleInput(pub String);

impl RawPuzzleInput {
    pub async fn fetch(session_token: &str, year: Year, day: Day) -> Result<RawPuzzleInput, Box<dyn Error>> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year.0, day.0);
        let client = reqwest::Client::new();
        let response_body = client
            .get(&url)
            .header(COOKIE, format!("session={}", session_token))
            .send().await
            .map_err(|error| format!("failed to send request: {}", error))?
            .text().await
            .map_err(|error| format!("failed to read response body: {}", error))?;


        Ok(RawPuzzleInput(response_body))
    }
}


#[derive(Debug, PartialEq)]
pub struct Part1Result(pub usize);

#[derive(Debug, PartialEq)]
pub struct Part2Result(pub usize);

#[derive(Debug, PartialEq)]
pub struct PuzzleResult {
    pub part1: Result<Part1Result, PuzzleError>,
    pub part2: Result<Part2Result, PuzzleError>,
}

impl Display for PuzzleResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.part1 {
            Ok(result) => writeln!(f, "Part 1: {}", result.0),
            Err(error) => writeln!(f, "Part 1: {}", error),
        }?;
        match &self.part2 {
            Ok(result) => writeln!(f, "Part 2: {}", result.0),
            Err(error) => writeln!(f, "Part 2: {}", error),
        }
    }
}

pub fn puzzle_not_implemented() -> PuzzleResult {
    PuzzleResult {
        part1: Err(PuzzleError::NotImplemented),
        part2: Err(PuzzleError::NotImplemented),
    }
}

pub trait Puzzle {

    fn part1(input : &RawPuzzleInput) -> Result<Part1Result, PuzzleError>;

    fn part2(input : &RawPuzzleInput) -> Result<Part2Result, PuzzleError>;

    fn solve(input: &RawPuzzleInput) -> PuzzleResult {
        PuzzleResult {
            part1: Self::part1(input),
            part2: Self::part2(input),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PuzzleError {
    NotImplemented,
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PuzzleError::NotImplemented => write!(f, "not implemented"),
        }
    }
}

impl Error for PuzzleError {}