use advent_of_code_template::{day19::Day19, solve_day, Solution};
fn parsing() {
    Day19::parse_input(include_str!("../inputs/19"));
}

fn parsing_and_part_one() {
    Day19::solve_part_one(include_str!("../inputs/19"));
}

fn parsing_and_part_two() {
    Day19::solve_part_two(include_str!("../inputs/19"));
}

fn whole_solution() {
    solve_day(&19, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
