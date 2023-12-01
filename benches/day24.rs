use advent_of_code_template::{day24::Day24, solve_day, Solution};
fn parsing() {
    Day24::parse_input(include_str!("../inputs/24"));
}

fn parsing_and_part_one() {
    Day24::solve_part_one(include_str!("../inputs/24"));
}

fn parsing_and_part_two() {
    Day24::solve_part_two(include_str!("../inputs/24"));
}

fn whole_solution() {
    solve_day(&24, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
