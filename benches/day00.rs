use advent_of_code_template::{day00::Day00, solve_day, Solution};
fn parsing() {
    Day00::parse_input(include_str!("../inputs/0"));
}

fn parsing_and_part_one() {
    Day00::solve_part_one(include_str!("../inputs/0"));
}

fn parsing_and_part_two() {
    Day00::solve_part_two(include_str!("../inputs/0"));
}

fn whole_solution() {
    solve_day(&0, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
