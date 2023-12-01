use advent_of_code_template::{day11::Day11, solve_day, Solution};
fn parsing() {
    Day11::parse_input(include_str!("../inputs/11"));
}

fn parsing_and_part_one() {
    Day11::solve_part_one(include_str!("../inputs/11"));
}

fn parsing_and_part_two() {
    Day11::solve_part_two(include_str!("../inputs/11"));
}

fn whole_solution() {
    solve_day(&11, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
