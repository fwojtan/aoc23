use advent_of_code_template::{day09::Day09, solve_day, Solution};
fn parsing() {
    Day09::parse_input(include_str!("../inputs/9"));
}

fn parsing_and_part_one() {
    Day09::solve_part_one(include_str!("../inputs/9"));
}

fn parsing_and_part_two() {
    Day09::solve_part_two(include_str!("../inputs/9"));
}

fn whole_solution() {
    solve_day(&9, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
