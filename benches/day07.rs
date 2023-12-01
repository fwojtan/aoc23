use advent_of_code_template::{day07::Day07, solve_day, Solution};
fn parsing() {
    Day07::parse_input(include_str!("../inputs/7"));
}

fn parsing_and_part_one() {
    Day07::solve_part_one(include_str!("../inputs/7"));
}

fn parsing_and_part_two() {
    Day07::solve_part_two(include_str!("../inputs/7"));
}

fn whole_solution() {
    solve_day(&7, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
