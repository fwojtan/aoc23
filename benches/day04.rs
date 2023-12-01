use advent_of_code_template::{day04::Day04, solve_day, Solution};
fn parsing() {
    Day04::parse_input(include_str!("../inputs/4"));
}

fn parsing_and_part_one() {
    Day04::solve_part_one(include_str!("../inputs/4"));
}

fn parsing_and_part_two() {
    Day04::solve_part_two(include_str!("../inputs/4"));
}

fn whole_solution() {
    solve_day(&4, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
