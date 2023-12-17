use std::error::Error;
use reqwest::header::COOKIE;
use crate::aoc::{Day, PuzzleInput, Year};

pub(crate) async fn fetch(session_token: &str, year: Year, day: Day) -> Result<PuzzleInput, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year.0, day.0);
    let client = reqwest::Client::new();
    let response_body = client
        .get(&url)
        .header(COOKIE, format!("session={}", session_token))
        .send().await
        .map_err(|error| format!("failed to send request: {}", error))?
        .text().await
        .map_err(|error| format!("failed to read response body: {}", error))?;


    Ok(PuzzleInput(response_body))
}