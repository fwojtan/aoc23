use advent_of_code_template::{day15::Day15, solve_day, Solution};
fn parsing() {
    Day15::parse_input(include_str!("../inputs/15"));
}

fn parsing_and_part_one() {
    Day15::solve_part_one(include_str!("../inputs/15"));
}

fn parsing_and_part_two() {
    Day15::solve_part_two(include_str!("../inputs/15"));
}

fn whole_solution() {
    solve_day(&15, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
