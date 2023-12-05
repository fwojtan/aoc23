use std::{iter::Peekable, num::ParseIntError, str::Chars};

use crate::Solution;

const WINNERS_LEN: usize = 10;
const MY_NUMBERS_LEN: usize = 25;

// const WINNERS_LEN:usize = 5;
// const MY_NUMBERS_LEN:usize = 8;

#[derive(Clone, Debug)]
pub struct Day04;

impl Solution for Day04 {
    type ParsedInput = Vec<Card>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.

        let mut winning_numbers = [0; WINNERS_LEN];
        let mut winners = Vec::new();
        let mut cards = Vec::new();

        for line in input_lines.lines() {
            winners.clear();

            let mut parser = line.chars().peekable().parser();
            parser.skip(5).unwrap();
            let _id = parser.parse_int().unwrap();
            parser.step_over(':').unwrap();

            for i in winning_numbers.iter_mut() {
                *i = parser.parse_int().unwrap();
            }

            parser.step_over('|').unwrap();

            for _ in 0..MY_NUMBERS_LEN {
                let number = parser.parse_int().unwrap();
                if winning_numbers.contains(&number) {
                    winners.push(number)
                }
            }

            cards.push(Card {
                winners: winners.clone(),
            })
        }

        cards
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(|card| {
                if !card.winners.is_empty() {
                    1 << (card.winners.len() - 1)
                } else {
                    0
                }
            })
            .map(|i| i)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let mut additional_scratchcards = vec![0; parsed_input.len()];
        for (i, card) in parsed_input.iter().enumerate() {
            for _ in 0..(additional_scratchcards[i] + 1) {
                for j in 0..card.winners.len() {
                    if let Some(value) = additional_scratchcards.get_mut(i + j + 1) {
                        *value += 1;
                    }
                }
            }
        }
        (additional_scratchcards.iter().sum::<usize>() + parsed_input.len()).to_string()
    }
}

pub struct Card {
    winners: Vec<u8>,
}

trait MakeParser<'a> {
    fn parser(self) -> Parser<'a>;
}

impl<'a> MakeParser<'a> for Peekable<Chars<'a>> {
    fn parser(self) -> Parser<'a> {
        Parser { chars: self }
    }
}

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn ignore_whitespace(&mut self) {
        while let Some(item) = self.chars.peek() {
            if item.is_whitespace() {
                self.chars.next();
            } else {
                return;
            }
        }
    }

    fn skip(&mut self, n: usize) -> Result<(), ()> {
        for _ in 0..n {
            if self.chars.next().is_none() {
                return Err(());
            };
        }
        Ok(())
    }

    fn step_over(&mut self, char_to_skip: char) -> Result<(), ()> {
        self.ignore_whitespace();
        if let Some(item) = self.chars.peek() {
            if char_to_skip == *item {
                self.chars.next().unwrap();
                return Ok(());
            }
        }
        Err(())
    }

    fn parse_int(&mut self) -> Result<u8, ParseIntError> {
        let mut chars_to_parse: String = "".to_string();

        self.ignore_whitespace();

        while let Some(item) = self.chars.peek() {
            if item.is_ascii_digit() {
                chars_to_parse.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        // If the first char isn't a digit this will return an error
        chars_to_parse.parse::<u8>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            Day04::solve_part_one(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "13".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(Day04::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(Day04::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
