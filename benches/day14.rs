use advent_of_code_template::{day14::Day14, solve_day, Solution};
fn parsing() {
    Day14::parse_input(include_str!("../inputs/14"));
}

fn parsing_and_part_one() {
    Day14::solve_part_one(include_str!("../inputs/14"));
}

fn parsing_and_part_two() {
    Day14::solve_part_two(include_str!("../inputs/14"));
}

fn whole_solution() {
    solve_day(&14, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
