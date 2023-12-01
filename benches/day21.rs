use advent_of_code_template::{day21::Day21, solve_day, Solution};
fn parsing() {
    Day21::parse_input(include_str!("../inputs/21"));
}

fn parsing_and_part_one() {
    Day21::solve_part_one(include_str!("../inputs/21"));
}

fn parsing_and_part_two() {
    Day21::solve_part_two(include_str!("../inputs/21"));
}

fn whole_solution() {
    solve_day(&21, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
