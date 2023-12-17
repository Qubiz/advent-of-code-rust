use crate::aoc::{Part1Result, Part2Result, Puzzle, PuzzleError, RawPuzzleInput};

pub(crate) struct Day1 {}

impl Puzzle for Day1 {
    fn part1(_input: &RawPuzzleInput) -> Result<Part1Result, PuzzleError> {
        Err(PuzzleError::NotImplemented)
    }

    fn part2(_input: &RawPuzzleInput) -> Result<Part2Result, PuzzleError> {
        Err(PuzzleError::NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::{Part1Result, Puzzle, RawPuzzleInput};
    use crate::year2023::day1::Day1;

    const EXAMPLE_INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn test_part1() {
        assert_eq!(Day1::part1(&RawPuzzleInput(EXAMPLE_INPUT.to_string())), Ok(Part1Result(0)));
    }

    #[test]
    fn test_part2() {
        todo!("Implement part 2");
    }
}
