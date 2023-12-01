use advent_of_code_template::{day06::Day06, solve_day, Solution};
fn parsing() {
    Day06::parse_input(include_str!("../inputs/6"));
}

fn parsing_and_part_one() {
    Day06::solve_part_one(include_str!("../inputs/6"));
}

fn parsing_and_part_two() {
    Day06::solve_part_two(include_str!("../inputs/6"));
}

fn whole_solution() {
    solve_day(&6, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
