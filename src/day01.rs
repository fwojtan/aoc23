use std::collections::VecDeque;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day01;

const NUMS: &str = "0123456789";
const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";

const WORD_NUMS: [&str; 9] = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

impl Solution for Day01 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let mut total = 0;
        for line in parsed_input.lines() {
            let (mut first, mut last) = (None, None);
            for val in line.chars() {
                if NUMS.contains(val) {
                    if first.is_none() {
                        first = Some(val);
                    }
                    last = Some(val);
                }
            }

            let f = first.expect("Line must contain digit").to_string();
            let l = last.expect("Line must contain digit").to_string();
            let value_str = vec![f, l].join("");

            total += value_str.parse::<u32>().expect("Value must be integer");
        }
        total.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let mut total = 0;
        for line in parsed_input.lines() {
            let (mut first, mut last) = (None, None);
            let mut recent_state = VecDeque::new();
            for val in line.chars() {
                if NUMS.contains(val) {
                    if first.is_none() {
                        first = Some(val);
                    }
                    last = Some(val);
                }
                recent_state.push_back(val);
                if let Some(val) = check_recent_state_for_number(&recent_state) {
                    if first.is_none() {
                        first = Some(val);
                    }
                    last = Some(val);
                }
            }

            let f = first.expect("Line must contain digit").to_string();
            let l = last.expect("Line must contain digit").to_string();
            let value_str = vec![f, l].join("");

            // println!("{} - {}", value_str, line);

            total += value_str.parse::<u32>().expect("Value must be integer");
        }
        total.to_string()
    }
}

fn check_recent_state_for_number(state: &VecDeque<char>) -> Option<char> {
    for (value, word) in (1..10).zip(WORD_NUMS) {
        if state.len() >= word.len() {
            let mut all_matching = true;
            for (char1, char2) in word.chars().rev().zip(state.iter().rev()) {
                if char1 != *char2 {
                    all_matching = false;
                    break;
                }
            }
            if all_matching {
                return Some(
                    value
                        .to_string()
                        .chars()
                        .next()
                        .expect("Should have one character in this int"),
                );
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(Day01::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(
            Day01::solve_part_two(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "281".to_string()
        )
    }

    #[test]
    fn check_day01_part2_case2() {
        assert_eq!(
            Day01::solve_part_two(
                "cmpptgjc3qhcjxcbcqgqkxhrms
9sixonefour
eighttwo2twofour9
7eightseveneightthree
tlnllks2jcfdlgsjbhpfnineone
one44fivesevenjzsfzddg
787seveneight75five
7q7nine3
hlqphphn5dtzzbqvk3three8seven"
            ),
            (33 + 94 + 89 + 73 + 21 + 17 + 75 + 73 + 57).to_string()
        )
    }

    #[test]
    fn check_day01_part2_case3() {
        assert_eq!(
            Day01::solve_part_two("sixfckdsbtmkj193onetwonexm"),
            61.to_string()
        )
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(Day01::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
