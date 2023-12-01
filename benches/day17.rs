use advent_of_code_template::{day17::Day17, solve_day, Solution};
fn parsing() {
    Day17::parse_input(include_str!("../inputs/17"));
}

fn parsing_and_part_one() {
    Day17::solve_part_one(include_str!("../inputs/17"));
}

fn parsing_and_part_two() {
    Day17::solve_part_two(include_str!("../inputs/17"));
}

fn whole_solution() {
    solve_day(&17, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
