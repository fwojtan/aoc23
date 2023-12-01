use advent_of_code_template::{day25::Day25, solve_day, Solution};
fn parsing() {
    Day25::parse_input(include_str!("../inputs/25"));
}

fn parsing_and_part_one() {
    Day25::solve_part_one(include_str!("../inputs/25"));
}

fn parsing_and_part_two() {
    Day25::solve_part_two(include_str!("../inputs/25"));
}

fn whole_solution() {
    solve_day(&25, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
