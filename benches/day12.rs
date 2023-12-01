use advent_of_code_template::{day12::Day12, solve_day, Solution};
fn parsing() {
    Day12::parse_input(include_str!("../inputs/12"));
}

fn parsing_and_part_one() {
    Day12::solve_part_one(include_str!("../inputs/12"));
}

fn parsing_and_part_two() {
    Day12::solve_part_two(include_str!("../inputs/12"));
}

fn whole_solution() {
    solve_day(&12, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
