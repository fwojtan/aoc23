use advent_of_code_template::{day16::Day16, solve_day, Solution};
fn parsing() {
    Day16::parse_input(include_str!("../inputs/16"));
}

fn parsing_and_part_one() {
    Day16::solve_part_one(include_str!("../inputs/16"));
}

fn parsing_and_part_two() {
    Day16::solve_part_two(include_str!("../inputs/16"));
}

fn whole_solution() {
    solve_day(&16, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
