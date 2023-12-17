use std::ops::Add;
use crate::aoc::{Part1Result, Part2Result, Puzzle, PuzzleError, RawPuzzleInput};

pub(crate) struct Day1 {}

impl Add<usize> for Part1Result {
    type Output = Part1Result;

    fn add(self, rhs: usize) -> Self::Output {
        Part1Result(self.0 + rhs)
    }
}

fn char_at(index: usize, line: &str) -> Option<char> {
    line.as_bytes().get(index).map(|b| { *b as char })
}

fn first_digit(line: &str) -> Option<char> {
    let index = line.find(|c: char| c.is_ascii_digit())?;
    char_at(index, line)
}

fn last_digit(line: &str) -> Option<char> {
    let index = line.rfind(|c: char| c.is_ascii_digit())?;
    char_at(index, line)
}

impl Puzzle for Day1 {
    fn part1(input: &RawPuzzleInput) -> Result<Part1Result, PuzzleError> {
        // For each line,
        let result = input.0.lines().fold(0, |acc, line| {
            let first_digit = first_digit(line).unwrap();
            let last_digit = last_digit(line).unwrap();
            let combined : usize = format!("{}{}", first_digit, last_digit).parse().unwrap();
            acc + combined
        });
        Ok(Part1Result(result))
    }

    fn part2(_input: &RawPuzzleInput) -> Result<Part2Result, PuzzleError> {
        Err(PuzzleError::NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{Part1Result, Puzzle, RawPuzzleInput};
    use crate::year2023::day1::{Day1, first_digit, last_digit};

    const EXAMPLE_INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn test_first_digit() {
        let mut lines = EXAMPLE_INPUT.lines();
        assert_eq!(Some('1'), first_digit(lines.next().unwrap()));
        assert_eq!(Some('3'), first_digit(lines.next().unwrap()));
        assert_eq!(Some('1'), first_digit(lines.next().unwrap()));
        assert_eq!(Some('7'), first_digit(lines.next().unwrap()));
    }


    #[test]
    fn test_last_digit() {
        let mut lines = EXAMPLE_INPUT.lines();
        assert_eq!(Some('2'), last_digit(lines.next().unwrap()));
        assert_eq!(Some('8'), last_digit(lines.next().unwrap()));
        assert_eq!(Some('5'), last_digit(lines.next().unwrap()));
        assert_eq!(Some('7'), last_digit(lines.next().unwrap()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day1::part1(&RawPuzzleInput(EXAMPLE_INPUT.to_string())), Ok(Part1Result(142)));
    }

    #[test]
    fn test_part2() {
        todo!("Implement part 2");
    }
}
