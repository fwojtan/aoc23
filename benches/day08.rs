use advent_of_code_template::{day08::Day08, solve_day, Solution};
fn parsing() {
    Day08::parse_input(include_str!("../inputs/8"));
}

fn parsing_and_part_one() {
    Day08::solve_part_one(include_str!("../inputs/8"));
}

fn parsing_and_part_two() {
    Day08::solve_part_two(include_str!("../inputs/8"));
}

fn whole_solution() {
    solve_day(&8, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
