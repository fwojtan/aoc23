use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day06;

impl Solution for Day06 {
    type ParsedInput = (Vec<(u16, u16)>, (u64, u64));

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut p2_dist = "".to_string();
        let mut p2_time = "".to_string();
        let mut lines = input_lines.lines();
        let p1_data = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .zip(lines.next().unwrap().split_ascii_whitespace())
            .skip(1)
            .map(|(time, dist)| {
                p2_dist += dist;
                p2_time += time;
                (time.parse::<u16>().unwrap(), dist.parse::<u16>().unwrap())
            })
            .collect::<Vec<(u16, u16)>>();
        (
            p1_data,
            (p2_time.parse().unwrap(), p2_dist.parse().unwrap()),
        )
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .0
            .iter()
            .map(|(time, dist)| {
                (0..*time)
                    .filter(move |i| {
                        let time_remaining = time - i;
                        time_remaining * i > *dist
                    })
                    .count() as u64
            })
            .product::<u64>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let time = parsed_input.1 .0;
        let dist = parsed_input.1 .1;
        (0..time)
            .filter(move |i| {
                let time_remaining = time - i;
                time_remaining * i > dist
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(Day06::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(Day06::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(Day06::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
