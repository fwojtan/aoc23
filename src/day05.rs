use std::{cmp::Ordering, sync::mpsc::channel};

use itertools::Itertools;
use rayon::prelude::*;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day05;

impl Solution for Day05 {
    type ParsedInput = (Vec<u64>, Vec<RangeMap>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut blocks = input_lines.split("\n\n");
        let seeds = blocks
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|num| num.parse::<u64>().expect("Parsing num"))
            .collect::<Vec<u64>>();

        let mut maps = Vec::new();
        for block in blocks {
            maps.push(RangeMap::new(
                block
                    .split("\n")
                    .skip(1)
                    .map(|line| {
                        line.split_ascii_whitespace()
                            .map(|num| num.parse::<u64>().expect("Parsing soil num"))
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect::<Vec<(u64, u64, u64)>>(),
            ));
        }
        (seeds, maps)
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        minimum_location_for_seeds(&parsed_input.0, &parsed_input.1).to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        // 1.2bn seeds in my input...
        // Part 1 takes 5us for 20 seeds so call it 0.25 us per seed
        // So a rough estimate is 300 seconds... so 5 minutes...?
        // I should parallelize
        // After switching to u64s this became even worse. I was close to
        // OOM on a 16GB system and this solution takes nearly 3 mintues to run... :(

        let mut seeds = Vec::new();
        let mut iter = parsed_input.0.iter();
        for _ in 0..(parsed_input.0.len() / 2) {
            let start = iter.next().unwrap();
            let length = iter.next().unwrap();
            for i in 0..*length {
                seeds.push(start + i)
            }
        }
        minimum_location_for_seeds(&seeds, &parsed_input.1).to_string()
    }
}

#[derive(Debug)]
pub struct RangeMap {
    _ranges: Vec<(u64, u64, u64)>,
}

impl RangeMap {
    fn new(mut data: Vec<(u64, u64, u64)>) -> Self {
        data.sort_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .expect("Comparison of two u64s should work")
        });
        RangeMap { _ranges: data }
    }

    fn get(&self, key: &u64) -> u64 {
        match self._ranges.binary_search_by(|a| {
            if key < &a.1 {
                Ordering::Greater
            } else if key >= &(a.1 + a.2) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(idx) => {
                let map = self._ranges[idx];
                map.0 + (key - map.1)
            }
            Err(_) => *key,
        }
    }
}

fn minimum_location_for_seeds(seeds: &Vec<u64>, maps: &Vec<RangeMap>) -> u64 {
    let (sender, receiver) = channel();
    let minimum_thread = std::thread::spawn(move || {
        let mut minimum = u64::MAX;
        loop {
            match receiver.recv() {
                Ok(value) => {
                    if value < minimum {
                        minimum = value;
                    }
                }
                Err(_) => break,
            }
        }
        minimum
    });
    seeds.into_par_iter().for_each_with(sender, |s, seed| {
        let mut current_value = seed.clone();
        for map in maps.iter() {
            current_value = map.get(&current_value);
        }
        s.send(current_value).unwrap();
    });
    minimum_thread.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(
            Day05::solve_part_one(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            "35".to_string()
        )
    }

    #[test]
    fn test_get() {
        let map = RangeMap::new(vec![(52, 50, 48)]);
        assert_eq!(map.get(&79), 81);
    }

    #[test]
    fn test_get_2() {
        let map = RangeMap::new(vec![(52, 50, 48), (50, 98, 2)]);
        assert_eq!(map.get(&79), 81);
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(
            Day05::solve_part_two(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            "46".to_string()
        )
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(Day05::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
