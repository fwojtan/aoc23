use advent_of_code_template::{day13::Day13, solve_day, Solution};
fn parsing() {
    Day13::parse_input(include_str!("../inputs/13"));
}

fn parsing_and_part_one() {
    Day13::solve_part_one(include_str!("../inputs/13"));
}

fn parsing_and_part_two() {
    Day13::solve_part_two(include_str!("../inputs/13"));
}

fn whole_solution() {
    solve_day(&13, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
