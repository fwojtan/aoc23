use std::{
    cmp::max,
    iter::Peekable,
    num::ParseIntError,
    str::{Chars, FromStr},
};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day02;

impl Solution for Day02 {
    type ParsedInput = Vec<Game>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut games = vec![];
        for line in input_lines.lines() {
            let game = Game::from_str(line).unwrap();
            games.push(game);
        }
        games
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .filter_map(|game| {
                if game.colour_max.0 <= 12 && game.colour_max.1 <= 13 && game.colour_max.2 <= 14 {
                    Some(game.id)
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(|game| game.colour_max.0 * game.colour_max.1 * game.colour_max.2)
            .sum::<u32>()
            .to_string()
    }
}

pub struct Game {
    id: u32,
    colour_max: (u32, u32, u32),
}

impl FromStr for Game {
    fn from_str(s: &str) -> Result<Game, ()> {
        let mut colour_max = (0, 0, 0);

        let mut parser = s.chars().peekable().parser();
        let _ = parser.skip(5);
        let id = parser.parse_int().unwrap();
        let _ = parser.step_over(':');

        'turns: loop {
            'colours: loop {
                let quantity = parser.parse_int().unwrap();
                let colour = parser.parse_word().unwrap();

                match colour.as_str() {
                    "red" => {
                        colour_max.0 = max(colour_max.0, quantity);
                    }
                    "green" => {
                        colour_max.1 = max(colour_max.1, quantity);
                    }
                    "blue" => {
                        colour_max.2 = max(colour_max.2, quantity);
                    }
                    _ => panic!("Parsed unexpected word"),
                }
                if parser.step_over(',').is_err() {
                    break 'colours;
                }
            }
            if parser.step_over(';').is_err() {
                break 'turns;
            }
        }

        Ok(Game { id, colour_max })
    }

    type Err = ();
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

    fn parse_int(&mut self) -> Result<u32, ParseIntError> {
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
        chars_to_parse.parse::<u32>()
    }

    fn parse_word(&mut self) -> Result<String, ()> {
        let mut word: String = "".to_string();

        self.ignore_whitespace();

        while let Some(item) = self.chars.peek() {
            if item.is_alphabetic() {
                word.push(self.chars.next().unwrap());
            } else if word.is_empty() {
                // Supplied characters did not start with a letter
                return Err(());
            } else {
                break;
            }
        }

        Ok(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(
            Day02::solve_part_one(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "8".to_string()
        )
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(Day02::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(Day02::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
