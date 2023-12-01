use advent_of_code_template::{day18::Day18, solve_day, Solution};
fn parsing() {
    Day18::parse_input(include_str!("../inputs/18"));
}

fn parsing_and_part_one() {
    Day18::solve_part_one(include_str!("../inputs/18"));
}

fn parsing_and_part_two() {
    Day18::solve_part_two(include_str!("../inputs/18"));
}

fn whole_solution() {
    solve_day(&18, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
