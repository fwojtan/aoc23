// use std::ops::Mul;

use crate::Solution;

// const N: usize = if cfg!(test) { 6 } else { 21 };

// static POLY_MATRIX: Lazy<DMatrix<f64>> =
//     Lazy::new(|| DMatrix::<f64>::from_fn(N, N, |i, j| pow((i + 1) as f64, j)));

// static INV_POLY_MATRIX: Lazy<DMatrix<f64>> =
//     Lazy::new(|| POLY_MATRIX.clone().try_inverse().unwrap());

#[derive(Clone, Debug)]
pub struct Day09;

impl Solution for Day09 {
    type ParsedInput = Vec<Vec<i64>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        // I had a rather lovely solution for the example data for part 1
        // I've included it commented below
        // Solved a system of linear equations to find the coefficients for
        // the general nth term based on the given sequence
        // This did *not* scale up well to the 21 terms given in the input...

        // let term_to_calculate = if cfg!(test) { 7i64 } else { 22i64 };
        // parsed_input
        //     .iter()
        //     .map(|sequence| {
        //         let short_sequence =
        //             DVector::<f64>::from_iterator(N, sequence.clone().into_iter().take(N));
        //         let terms = INV_POLY_MATRIX
        //             .clone()
        //             .mul(short_sequence)
        //             .iter()
        //             .map(|i| *i as f64)
        //             .collect::<Vec<_>>();
        //         terms
        //             .iter()
        //             .enumerate()
        //             .map(|(degree, coefficient)| {
        //                 pow(term_to_calculate, degree) as f64 * *coefficient
        //             })
        //             .sum::<f64>()
        //     })
        //     .map(|nex_term| { nex_term })
        //     .sum::<f64>()
        //     .round()
        //     .to_string()

        parsed_input
            .iter()
            .map(|seq| {
                let mut levels = vec![];
                let mut current_seq = seq.iter().rev().copied().collect::<Vec<_>>();
                loop {
                    let mut last: Option<i64> = None;
                    let mut diffs = vec![];
                    for val in current_seq.iter() {
                        if let Some(last_val) = last {
                            diffs.push(last_val - val);
                            last = Some(*val);
                        } else {
                            last = Some(*val);
                        }
                    }
                    if diffs.iter().all(|diff| *diff == 0i64) {
                        break;
                    } else {
                        current_seq = diffs.clone();
                        levels.push(diffs);
                    }
                }
                levels.iter().map(|level| level[0]).sum::<i64>() + *seq.iter().last().unwrap()
            })
            .sum::<i64>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(|seq| {
                let mut levels = vec![];
                let mut current_seq = seq.iter().rev().copied().collect::<Vec<_>>();
                loop {
                    let mut last: Option<i64> = None;
                    let mut diffs = vec![];
                    for val in current_seq.iter() {
                        if let Some(last_val) = last {
                            diffs.push(last_val - val);
                            last = Some(*val);
                        } else {
                            last = Some(*val);
                        }
                    }
                    if diffs.iter().all(|diff| *diff == 0i64) {
                        break;
                    } else {
                        current_seq = diffs.clone();
                        levels.push(diffs);
                    }
                }
                let mut next_for_seq = 0;
                for level in levels.iter().rev() {
                    next_for_seq = level.iter().last().unwrap() - next_for_seq;
                }
                seq[0] - next_for_seq
            })
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(
            Day09::solve_part_one(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "114".to_string()
        )
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(
            Day09::solve_part_two(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(Day09::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
