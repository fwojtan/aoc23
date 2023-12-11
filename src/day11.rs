use std::collections::HashMap;

use petgraph::{algo::dijkstra, dot::Dot, stable_graph::NodeIndex, Graph, Undirected};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day11;

pub struct Day11Data {
    graph: Graph<char, u64, Undirected>,
    galaxies: Vec<NodeIndex>,
    node_indices: Vec<Vec<NodeIndex>>,
    rows_to_expand: Vec<usize>,
    cols_to_expand: Vec<usize>,
}

impl Solution for Day11 {
    type ParsedInput = Day11Data;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let universe = input_lines
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut col_maj_universe = vec![Vec::with_capacity(universe.len()); universe.len()];
        for (row_idx, row) in universe.iter().enumerate() {
            for (col_idx, item) in row.iter().enumerate() {
                col_maj_universe[col_idx].push(*item)
            }
        }

        let mut graph = Graph::<char, u64, Undirected>::new_undirected();
        let mut galaxies = Vec::new();

        let mut node_indices = vec![Vec::with_capacity(universe[0].len()); universe.len()];

        for (idx, row) in universe.iter().enumerate() {
            for item in row {
                let node = graph.add_node(*item);
                if *item == '#' {
                    galaxies.push(node);
                }
                node_indices[idx].push(node);
            }
        }
        let rows_to_expand = universe
            .iter()
            .enumerate()
            .filter(|(_row_idx, row)| !row.contains(&'#'))
            .map(|(row_idx, _row)| row_idx)
            .collect::<Vec<_>>();

        let cols_to_expand = col_maj_universe
            .iter()
            .enumerate()
            .filter(|(_col_idx, col)| !col.contains(&'#'))
            .map(|(col_idx, _col)| col_idx)
            .collect::<Vec<_>>();

        for (col_idx, col) in node_indices.iter().enumerate() {
            for (row_idx, item) in col.iter().enumerate() {
                if let Some(next_col) = node_indices.get(col_idx + 1) {
                    graph.update_edge(*item, next_col[row_idx], 1);
                }
                if let Some(next_item) = col.get(row_idx + 1) {
                    graph.update_edge(*item, *next_item, 1);
                }
            }
        }

        Day11Data {
            graph,
            galaxies,
            node_indices,
            rows_to_expand,
            cols_to_expand,
        }
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        solve_for_expansion(parsed_input, 2)
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        solve_for_expansion(parsed_input, 1000000)
    }
}

fn solve_for_expansion(parsed_input: &mut Day11Data, expansion_amount: u64) -> String {
    for row_idx in &parsed_input.rows_to_expand {
        for (col_idx, node_idx_a) in parsed_input.node_indices[*row_idx].iter().enumerate() {
            parsed_input.graph.update_edge(
                *node_idx_a,
                parsed_input.node_indices[row_idx + 1][col_idx],
                expansion_amount,
            );
        }
    }

    for col_idx in &parsed_input.cols_to_expand {
        for row in parsed_input.node_indices.iter() {
            parsed_input
                .graph
                .update_edge(row[*col_idx], row[col_idx + 1], expansion_amount);
        }
    }

    let mut pairings = HashMap::new();
    for galaxy in parsed_input.galaxies.iter() {
        let distances = dijkstra(&parsed_input.graph, *galaxy, None, |e| *e.weight() as u64);
        for (target_node, distance) in distances.into_iter() {
            if *parsed_input.graph.node_weight(target_node).unwrap() == '#' {
                pairings.insert(
                    (*galaxy.min(&target_node), *galaxy.max(&target_node)),
                    distance,
                );
            }
        }
    }
    pairings.values().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day11_part1_case1() {
        assert_eq!(
            Day11::solve_part_one(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            "374".to_string()
        )
    }

    #[test]
    fn check_day11_case2() {
        assert_eq!(
            solve_for_expansion(
                &mut Day11::parse_input(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                10
            ),
            "1030"
        )
    }

    #[test]
    fn check_day11_case3() {
        assert_eq!(
            solve_for_expansion(
                &mut Day11::parse_input(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                100
            ),
            "8410"
        )
    }

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(Day11::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
