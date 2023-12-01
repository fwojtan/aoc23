use advent_of_code_template::{day02::Day02, solve_day, Solution};
fn parsing() {
    Day02::parse_input(include_str!("../inputs/2"));
}

fn parsing_and_part_one() {
    Day02::solve_part_one(include_str!("../inputs/2"));
}

fn parsing_and_part_two() {
    Day02::solve_part_two(include_str!("../inputs/2"));
}

fn whole_solution() {
    solve_day(&2, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
