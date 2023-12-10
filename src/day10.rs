use std::collections::{HashMap, HashSet};

use nalgebra::Vector2;
use petgraph::{
    algo::{dijkstra, has_path_connecting},
    dot::{Config, Dot},
    stable_graph::NodeIndex,
    visit::{Bfs, EdgeRef},
    Graph, Undirected,
};

use crate::Solution;

const NORTH: Vector2<isize> = Vector2::new(0, -1);
const SOUTH: Vector2<isize> = Vector2::new(0, 1);
const EAST: Vector2<isize> = Vector2::new(1, 0);
const WEST: Vector2<isize> = Vector2::new(-1, 0);

#[derive(Clone, Debug)]
pub struct Day10;

impl Solution for Day10 {
    type ParsedInput = (Graph<Node, (), Undirected>, NodeIndex);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.

        let mut graph = Graph::<Node, (), Undirected>::new_undirected();
        let mut pipe_loop = Graph::<Node, (), Undirected>::new_undirected();
        let mut coord_map: HashMap<Vector2<isize>, NodeIndex> = HashMap::new();
        let mut start = None;

        for (y, row) in input_lines.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let node = Node::new(x, y, c);

                match node.pipe_type {
                    PipeType::NotPipe => (),
                    PipeType::Unknown => {
                        // This is the starting location, save node idx
                        let node_idx = graph.add_node(node.clone());
                        coord_map.insert(node.coord, node_idx);
                        start = Some(node_idx);
                    }
                    _ => {
                        let node_idx = graph.add_node(node.clone());
                        coord_map.insert(node.coord, node_idx);
                    }
                }
            }
        }

        let mut adj_coords = vec![];
        for (coord, node_idx) in coord_map.iter() {
            adj_coords.clear();
            match graph
                .node_weight(*node_idx)
                .expect("Everything should be in graph")
                .pipe_type
            {
                PipeType::NotPipe => panic!("Should store no empty space"),
                PipeType::NE => {
                    if check_north(coord, &coord_map, &graph) {
                        adj_coords.push(coord + NORTH)
                    }
                    if check_east(coord, &coord_map, &graph) {
                        adj_coords.push(coord + EAST)
                    }
                }
                PipeType::NW => {
                    if check_north(coord, &coord_map, &graph) {
                        adj_coords.push(coord + NORTH)
                    }
                    if check_west(coord, &coord_map, &graph) {
                        adj_coords.push(coord + WEST)
                    }
                }
                PipeType::SE => {
                    if check_south(coord, &coord_map, &graph) {
                        adj_coords.push(coord + SOUTH)
                    }
                    if check_east(coord, &coord_map, &graph) {
                        adj_coords.push(coord + EAST)
                    }
                }
                PipeType::SW => {
                    if check_south(coord, &coord_map, &graph) {
                        adj_coords.push(coord + SOUTH)
                    }
                    if check_west(coord, &coord_map, &graph) {
                        adj_coords.push(coord + WEST)
                    }
                }
                PipeType::Horizontal => {
                    if check_west(coord, &coord_map, &graph) {
                        adj_coords.push(coord + WEST)
                    }
                    if check_east(coord, &coord_map, &graph) {
                        adj_coords.push(coord + EAST)
                    }
                }
                PipeType::Vertical => {
                    if check_south(coord, &coord_map, &graph) {
                        adj_coords.push(coord + SOUTH)
                    }
                    if check_north(coord, &coord_map, &graph) {
                        adj_coords.push(coord + NORTH)
                    }
                }
                PipeType::Unknown => {
                    if check_west(coord, &coord_map, &graph) {
                        adj_coords.push(coord + WEST)
                    }
                    if check_east(coord, &coord_map, &graph) {
                        adj_coords.push(coord + EAST)
                    }
                    if check_south(coord, &coord_map, &graph) {
                        adj_coords.push(coord + SOUTH)
                    }
                    if check_north(coord, &coord_map, &graph) {
                        adj_coords.push(coord + NORTH)
                    }
                }
                PipeType::InLoop => panic!(),
            };

            for adj_coord in &adj_coords {
                if let Some(neighbour) = coord_map.get(&adj_coord) {
                    graph.update_edge(*node_idx, *neighbour, ());
                }
            }
        }

        (graph, start.unwrap())
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        dijkstra(&parsed_input.0, parsed_input.1, None, |_| 1)
            .into_iter()
            .map(|(_idx, dist)| dist)
            .max()
            .unwrap()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let mut connected_nodes = HashSet::new();
        let mut bfs = Bfs::new(&parsed_input.0, parsed_input.1);
        while let Some(node_idx) = bfs.next(&parsed_input.0) {
            connected_nodes.insert(parsed_input.0.node_weight(node_idx).unwrap());
        }

        const GRID_SIZE: usize = if !cfg!(test) { 140 } else { 5 };
        let mut grid = [[PipeType::NotPipe; GRID_SIZE]; GRID_SIZE];

        for node in connected_nodes {
            grid[node.coord.y as usize][node.coord.x as usize] = node.pipe_type;
        }

        // Credit to reddit for helping me get this algorithm... I'd spent far too long on this puzzle already..
        for row in grid.iter_mut() {
            let mut currently_in_loop = false;
            for pipe_type in row.iter_mut() {
                match *pipe_type {
                    PipeType::NotPipe if currently_in_loop => *pipe_type = PipeType::InLoop,
                    PipeType::NE | PipeType::NW | PipeType::Vertical | PipeType::Unknown => {
                        currently_in_loop = !currently_in_loop
                    }
                    _ => (),
                }
            }
        }

        for row in grid {
            println!("{}", row.iter().map(|pt| pt.to_char()).collect::<String>());
        }

        grid.iter()
            .map(|row| row.iter().filter(|pt| **pt == PipeType::InLoop).count())
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Node {
    coord: Vector2<isize>,
    pipe_type: PipeType,
}

impl Node {
    fn new(x: usize, y: usize, c: char) -> Self {
        Node {
            coord: Vector2::new(x as isize, y as isize),
            pipe_type: PipeType::from_char(c),
        }
    }
}

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
pub enum PipeType {
    NotPipe,
    NE,
    NW,
    SE,
    SW,
    Horizontal,
    Vertical,
    Unknown,
    InLoop,
}

impl PipeType {
    fn from_char(c: char) -> PipeType {
        match c {
            '.' => PipeType::NotPipe,
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'L' => PipeType::NE,
            'J' => PipeType::NW,
            '7' => PipeType::SW,
            'F' => PipeType::SE,
            'S' => PipeType::Unknown,
            input => panic!("Unexpected input char: {}", input),
        }
    }

    fn to_char(&self) -> char {
        match self {
            PipeType::NotPipe => '.',
            PipeType::NE => '└',
            PipeType::NW => '┘',
            PipeType::SE => '┌',
            PipeType::SW => '┐',
            PipeType::Horizontal => '─',
            PipeType::Vertical => '│',
            PipeType::Unknown => 'S',
            PipeType::InLoop => 'x',
        }
    }

    fn is_connected_north(&self, other: &PipeType) -> bool {
        match self {
            PipeType::NotPipe | PipeType::SE | PipeType::SW | PipeType::Horizontal => return false,
            _ => (),
        }
        match other {
            PipeType::NotPipe | PipeType::NE | PipeType::NW | PipeType::Horizontal => false,
            _ => true,
        }
    }

    fn is_connected_south(&self, other: &PipeType) -> bool {
        match self {
            PipeType::NotPipe | PipeType::NE | PipeType::NW | PipeType::Horizontal => return false,
            _ => (),
        }
        match other {
            PipeType::NotPipe | PipeType::SE | PipeType::SW | PipeType::Horizontal => false,
            _ => true,
        }
    }

    fn is_connected_east(&self, other: &PipeType) -> bool {
        match self {
            PipeType::NotPipe | PipeType::NW | PipeType::SW | PipeType::Vertical => return false,
            _ => (),
        }
        match other {
            PipeType::NotPipe | PipeType::NE | PipeType::SE | PipeType::Vertical => false,
            _ => true,
        }
    }

    fn is_connected_west(&self, other: &PipeType) -> bool {
        match self {
            PipeType::NotPipe | PipeType::NE | PipeType::SE | PipeType::Vertical => return false,
            _ => (),
        }
        match other {
            PipeType::NotPipe | PipeType::NW | PipeType::SW | PipeType::Vertical => false,
            _ => true,
        }
    }
}

fn check_north(
    coord: &Vector2<isize>,
    coord_map: &HashMap<Vector2<isize>, NodeIndex>,
    graph: &Graph<Node, (), Undirected>,
) -> bool {
    if let Some(adj) = coord_map.get(&(coord + NORTH)) {
        graph
            .node_weight(*coord_map.get(coord).unwrap())
            .unwrap()
            .pipe_type
            .is_connected_north(&graph.node_weight(*adj).unwrap().pipe_type)
    } else {
        false
    }
}

fn check_south(
    coord: &Vector2<isize>,
    coord_map: &HashMap<Vector2<isize>, NodeIndex>,
    graph: &Graph<Node, (), Undirected>,
) -> bool {
    if let Some(adj) = coord_map.get(&(coord + SOUTH)) {
        graph
            .node_weight(*coord_map.get(coord).unwrap())
            .unwrap()
            .pipe_type
            .is_connected_south(&graph.node_weight(*adj).unwrap().pipe_type)
    } else {
        false
    }
}

fn check_east(
    coord: &Vector2<isize>,
    coord_map: &HashMap<Vector2<isize>, NodeIndex>,
    graph: &Graph<Node, (), Undirected>,
) -> bool {
    if let Some(adj) = coord_map.get(&(coord + EAST)) {
        graph
            .node_weight(*coord_map.get(coord).unwrap())
            .unwrap()
            .pipe_type
            .is_connected_east(&graph.node_weight(*adj).unwrap().pipe_type)
    } else {
        false
    }
}

fn check_west(
    coord: &Vector2<isize>,
    coord_map: &HashMap<Vector2<isize>, NodeIndex>,
    graph: &Graph<Node, (), Undirected>,
) -> bool {
    if let Some(adj) = coord_map.get(&(coord + WEST)) {
        graph
            .node_weight(*coord_map.get(coord).unwrap())
            .unwrap()
            .pipe_type
            .is_connected_west(&graph.node_weight(*adj).unwrap().pipe_type)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day10_part1_case1() {
        assert_eq!(
            Day10::solve_part_one(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day10_part1_case2() {
        assert_eq!(
            Day10::solve_part_one(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            "8".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(
            Day10::solve_part_two(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            "1".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case2() {
        assert_eq!(
            Day10::solve_part_two(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            "1".to_string()
        )
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(Day10::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
