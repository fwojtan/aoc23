use std::collections::HashMap;

use petgraph::{Graph, Undirected};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day03;

impl Solution for Day03 {
    type ParsedInput = Graph<Node, (), Undirected>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.

        const YSIZE: usize = 140;
        const XSIZE: usize = 140;

        let mut grid = [['.'; XSIZE]; YSIZE];

        for (i_y, line) in input_lines.lines().enumerate() {
            for (i_x, character) in line.chars().enumerate() {
                grid[i_y][i_x] = character;
            }
        }

        let mut graph = Graph::<Node, (), Undirected>::new_undirected();
        let mut symbol_indexes = HashMap::new();

        // Previously I allocated these where I call clear on them.
        // Moving them out of the loop made my whole solution 11% faster
        let mut adjacents = Vec::new(); // For collecting adjacent characters and their coords
        let mut adjacent_node_idxs = Vec::new(); // For collecting the graph node indices of adjacent characters

        for (i_y, row) in grid.iter().enumerate() {
            let mut value = 0;
            adjacents.clear();
            for (i_x, character) in row.iter().enumerate() {
                if character.is_ascii_digit() {
                    value += character.to_digit(10).unwrap();

                    for idx in adjacent_idxs((i_x, i_y)) {
                        if let Some(row) = grid.get(idx.1) {
                            if let Some(character) = row.get(idx.0) {
                                if *character != '.' && !character.is_ascii_digit() {
                                    adjacents.push((character, idx));
                                }
                            }
                        }
                    }

                    // Check if we're at the end of the number, if so add to graph
                    if row.get(i_x + 1).is_none()
                        || row.get(i_x + 1).is_some_and(|c| !c.is_ascii_digit())
                    {
                        adjacent_node_idxs.clear();
                        for adjacent_symbol in adjacents.iter() {
                            let index =
                                symbol_indexes
                                    .entry(adjacent_symbol.1)
                                    .or_insert(graph.add_node(Node {
                                        number: None,
                                        symbol: Some(*adjacent_symbol.0),
                                    }));
                            adjacent_node_idxs.push(*index);
                        }
                        let number_idx = graph.add_node(Node {
                            number: Some(value),
                            symbol: None,
                        });
                        for adj_idx in &adjacent_node_idxs {
                            graph.update_edge(number_idx, *adj_idx, ());
                        }

                        adjacents.clear();
                        value = 0;
                    } else {
                        // Next value is a digit so carry on accumulating adjacent values
                        value *= 10;
                    }
                }
            }
        }

        graph
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .node_indices()
            .filter_map(|node_idx| {
                if parsed_input[node_idx].is_num() && parsed_input.neighbors(node_idx).count() != 0
                {
                    Some(parsed_input[node_idx].number.unwrap())
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .node_indices()
            .filter(|node_idx| parsed_input[*node_idx].is_star())
            .filter_map(|node_idx| {
                if parsed_input
                    .neighbors(node_idx)
                    .filter(|neighbour_idx| parsed_input[*neighbour_idx].is_num())
                    .count()
                    == 2
                {
                    let gear_ratio: u32 = parsed_input
                        .neighbors(node_idx)
                        .filter(|neighbour_idx| parsed_input[*neighbour_idx].is_num())
                        .filter_map(|neighbour_idx| {
                            if parsed_input[neighbour_idx].is_num() {
                                Some(parsed_input[neighbour_idx].number.unwrap())
                            } else {
                                None
                            }
                        })
                        .product();
                    Some(gear_ratio)
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string()
    }
}

fn adjacent_idxs(coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut idxs = vec![
        (coord.0 + 1, coord.1 + 1),
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 + 1),
    ];
    if coord.0 > 0 {
        idxs.push((coord.0 - 1, coord.1 + 1));
        idxs.push((coord.0 - 1, coord.1));
        if coord.1 > 0 {
            idxs.push((coord.0 - 1, coord.1 - 1));
        }
    }
    if coord.1 > 0 {
        idxs.push((coord.0 + 1, coord.1 - 1));
        idxs.push((coord.0, coord.1 - 1));
    }
    idxs
}

#[derive(Debug)]
pub struct Node {
    number: Option<u32>,
    symbol: Option<char>,
}

impl Node {
    fn is_num(&self) -> bool {
        self.number.is_some()
    }

    fn is_star(&self) -> bool {
        if let Some(symbol) = self.symbol {
            if symbol == '*' {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            Day03::solve_part_one(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "4361".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            Day03::solve_part_two(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "467835".to_string()
        )
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(Day03::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
