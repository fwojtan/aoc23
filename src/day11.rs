use std::collections::HashMap;

use petgraph::{algo::dijkstra, stable_graph::NodeIndex, Graph, Undirected};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day11;

impl Solution for Day11 {
    type ParsedInput = (Graph<char, (), Undirected>, Vec<NodeIndex>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let row_major_universe = input_lines
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut partially_expanded_row_major_universe =
            Vec::with_capacity(row_major_universe.len() * 2);
        for row in row_major_universe {
            if !row.contains(&'#') {
                partially_expanded_row_major_universe.push(row.clone());
            }
            partially_expanded_row_major_universe.push(row);
        }

        let mut partially_expanded_column_major_universe =
            vec![
                Vec::with_capacity(partially_expanded_row_major_universe.len());
                partially_expanded_row_major_universe[0].len() * 2
            ];

        for row in partially_expanded_row_major_universe {
            for (idx, item) in row.iter().enumerate() {
                partially_expanded_column_major_universe[idx].push(*item)
            }
        }

        let mut expanded_universe =
            Vec::with_capacity(partially_expanded_column_major_universe.len() * 2);

        for col in partially_expanded_column_major_universe {
            if !col.contains(&'#') {
                expanded_universe.push(col.clone());
            }
            expanded_universe.push(col);
        }

        let expanded_universe: Vec<_> = expanded_universe
            .into_iter()
            .filter(|col| !col.is_empty())
            .collect();

        let mut graph = Graph::<char, (), Undirected>::new_undirected();
        let mut galaxies = Vec::new();

        let mut node_indices =
            vec![Vec::with_capacity(expanded_universe[0].len()); expanded_universe.len()];

        println!("{:?}", expanded_universe);

        for (idx, col) in expanded_universe.iter().enumerate() {
            for item in col {
                let node = graph.add_node(*item);
                if *item == '#' {
                    galaxies.push(node);
                }
                node_indices[idx].push(node);
            }
        }
        // println!("{:?}", node_indices);

        for (col_idx, col) in node_indices.iter().enumerate() {
            for (row_idx, item) in col.iter().enumerate() {
                if let Some(next_col) = node_indices.get(col_idx + 1) {
                    graph.update_edge(*item, next_col[row_idx], ());
                }
                if let Some(next_item) = col.get(row_idx + 1) {
                    graph.update_edge(*item, *next_item, ());
                }
            }
        }

        (graph, galaxies)
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let mut pairings = HashMap::new();
        println!("Number of galaxies: {}", parsed_input.1.len());

        for galaxy in parsed_input.1.iter() {
            let distances = dijkstra(&parsed_input.0, *galaxy, None, |_| 1);
            for (target_node, distance) in distances.into_iter() {
                if *parsed_input.0.node_weight(target_node).unwrap() == '#' {
                    pairings.insert(
                        (*galaxy.min(&target_node), *galaxy.max(&target_node)),
                        distance,
                    );
                }
            }
        }
        pairings.values().sum::<i32>().to_string()
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
    fn check_day11_part2_case1() {
        assert_eq!(Day11::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(Day11::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
