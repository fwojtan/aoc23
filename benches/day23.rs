use advent_of_code_template::{day23::Day23, solve_day, Solution};
fn parsing() {
    Day23::parse_input(include_str!("../inputs/23"));
}

fn parsing_and_part_one() {
    Day23::solve_part_one(include_str!("../inputs/23"));
}

fn parsing_and_part_two() {
    Day23::solve_part_two(include_str!("../inputs/23"));
}

fn whole_solution() {
    solve_day(&23, false)
}

iai::main!(
    parsing,
    parsing_and_part_one,
    parsing_and_part_two,
    whole_solution,
);
