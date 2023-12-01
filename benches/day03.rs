use advent_of_code_template::{day03::Day03, solve_day, Solution};
fn parsing() {
    Day03::parse_input(include_str!("../inputs/3"));
}

fn parsing_and_part_one() {
    Day03::solve_part_one(include_str!("../inputs/3"));
}

fn parsing_and_part_two() {
    Day03::solve_part_two(include_str!("../inputs/3"));
}

fn whole_solution() {
    solve_day(&3, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
