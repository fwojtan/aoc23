use advent_of_code_template::{day10::Day10, solve_day, Solution};
fn parsing() {
    Day10::parse_input(include_str!("../inputs/10"));
}

fn parsing_and_part_one() {
    Day10::solve_part_one(include_str!("../inputs/10"));
}

fn parsing_and_part_two() {
    Day10::solve_part_two(include_str!("../inputs/10"));
}

fn whole_solution() {
    solve_day(&10, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
