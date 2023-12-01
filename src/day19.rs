use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day19;

impl Solution for Day19 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part one
        0.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day19_part1_case1() {
        assert_eq!(Day19::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day19_part2_case1() {
        assert_eq!(Day19::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day19_both_case1() {
        assert_eq!(Day19::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
