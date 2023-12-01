use advent_of_code_template::{day05::Day05, solve_day, Solution};
fn parsing() {
    Day05::parse_input(include_str!("../inputs/5"));
}

fn parsing_and_part_one() {
    Day05::solve_part_one(include_str!("../inputs/5"));
}

fn parsing_and_part_two() {
    Day05::solve_part_two(include_str!("../inputs/5"));
}

fn whole_solution() {
    solve_day(&5, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
