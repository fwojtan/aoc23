use std::{collections::HashMap, iter::Peekable, str::Chars};

use clap::parser;
use num::Integer;
use petgraph::{
    stable_graph::NodeIndex,
    visit::{EdgeRef, IntoNeighbors},
    Directed, Graph,
};
use rayon::vec;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day08;

impl Solution for Day08 {
    type ParsedInput = (
        Vec<Direction>,
        Graph<String, Direction, Directed>,
        NodeIndex,
    );

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut lines = input_lines.lines();
        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| Direction::from_char(&c))
            .collect::<Vec<_>>();

        let mut graph = Graph::<String, Direction, Directed>::new();
        let mut node_indices = HashMap::new();

        let mut starting_index = None;
        lines.next();

        for line in lines {
            let mut parser = line.chars().peekable().parser();
            let node = parser.parse_word().unwrap();
            let _ = parser.step_over('=').unwrap();
            let _ = parser.step_over('(').unwrap();
            let left = parser.parse_word().unwrap();
            let _ = parser.step_over(',').unwrap();
            let right = parser.parse_word().unwrap();

            let node_idx = *node_indices
                .entry(node.clone())
                .or_insert(graph.add_node(node.clone()));
            let left_idx = *node_indices
                .entry(left.clone())
                .or_insert(graph.add_node(left));
            let right_idx = *node_indices
                .entry(right.clone())
                .or_insert(graph.add_node(right));

            graph.add_edge(node_idx, left_idx, Direction::Left);
            graph.add_edge(node_idx, right_idx, Direction::Right);
            if node == "AAA".to_string() {
                starting_index = Some(node_idx);
            }
        }

        (directions, graph, starting_index.unwrap())
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let directions = &parsed_input.0;
        let graph = &parsed_input.1;
        let mut count = 0;
        let mut current_node_idx = parsed_input.2;
        for direction in directions.iter().cycle() {
            count += 1;
            current_node_idx = graph
                .edges(current_node_idx)
                .find(|edge| edge.weight() == direction)
                .map(|edge| edge.target())
                .unwrap();
            if *graph.node_weight(current_node_idx).unwrap() == "ZZZ".to_string() {
                break;
            }
        }
        count.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let directions = &parsed_input.0;
        let graph = &parsed_input.1;

        let mut nodes = graph
            .node_indices()
            .filter(|node| graph.node_weight(*node).unwrap().ends_with("A"))
            .collect::<Vec<_>>();
        let mut count: u64 = 0;

        let mut step_counts = vec![None; nodes.len()];

        for direction in directions.iter().cycle() {
            count += 1;

            for (i, current_node_idx) in nodes.iter_mut().enumerate() {
                *current_node_idx = graph
                    .edges(*current_node_idx)
                    .find(|edge| edge.weight() == direction)
                    .map(|edge| edge.target())
                    .unwrap();
                if graph.node_weight(*current_node_idx).unwrap().ends_with("Z") {
                    step_counts[i] = Some(count);
                }
            }
            if step_counts.iter().all(|opt| opt.is_some()) {
                break;
            }
        }
        step_counts
            .iter()
            .map(|a| a.unwrap())
            .reduce(|a, b| a.lcm(&b))
            .unwrap()
            .to_string()
    }
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: &char) -> Direction {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!(),
        }
    }
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
    fn check_day08_part1_case1() {
        assert_eq!(
            Day08::solve_part_one(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day08_part1_case2() {
        assert_eq!(
            Day08::solve_part_one(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(Day08::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(Day08::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
