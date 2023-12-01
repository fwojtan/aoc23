use advent_of_code_template::{day20::Day20, solve_day, Solution};
fn parsing() {
    Day20::parse_input(include_str!("../inputs/20"));
}

fn parsing_and_part_one() {
    Day20::solve_part_one(include_str!("../inputs/20"));
}

fn parsing_and_part_two() {
    Day20::solve_part_two(include_str!("../inputs/20"));
}

fn whole_solution() {
    solve_day(&20, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
