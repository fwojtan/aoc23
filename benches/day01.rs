use advent_of_code_template::{day01::Day01, solve_day, Solution};
fn parsing() {
    Day01::parse_input(include_str!("../inputs/1"));
}

fn parsing_and_part_one() {
    Day01::solve_part_one(include_str!("../inputs/1"));
}

fn parsing_and_part_two() {
    Day01::solve_part_two(include_str!("../inputs/1"));
}

fn whole_solution() {
    solve_day(&1, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
