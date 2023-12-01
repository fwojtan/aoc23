use advent_of_code_template::{day22::Day22, solve_day, Solution};
fn parsing() {
    Day22::parse_input(include_str!("../inputs/22"));
}

fn parsing_and_part_one() {
    Day22::solve_part_one(include_str!("../inputs/22"));
}

fn parsing_and_part_two() {
    Day22::solve_part_two(include_str!("../inputs/22"));
}

fn whole_solution() {
    solve_day(&22, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
