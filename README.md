# AdventOfCodeTemplate

A sample template for Rust solutions to any one year of [Advent of Code](https://adventofcode.com/).

Adapted by Finlay Wojtan from a [previous template](https://github.com/CastleQuirm/AdventOfCodeTemplate) by Simon Castle (which was itself adapted from a previous template by Chris Paterson).

## How to Use

### Get setup
1. Set up your Rust environment
2. Clone this template (`git clone https://github.com/fwojtan/AoC-bench-template.git`)
3. Test you can run the code, using the Day 00 example.
    - In the terminal, go to the directory you've cloned the repo into (the directory containing this README.md file)
    - Run `cargo run 0`
        - This should show some build output (the first time this is run), followed by 
        > Day 0
        >
        > Part 1: 5971
        >
        > Part 2: 1155077
        >
        > 0.024ms (exact time may vary)
        > \----------
    - Run `cargo test 00`
        - This should show some build output (the first time this is run), followed by 
        > running 3 tests
        >
        > test day00::tests::check_day00_both_case1 ... ok
        >
        > test day00::tests::check_day00_part1_case1 ... ok
        >
        > test day00::tests::check_day00_part2_case1 ... ok
        >
        > test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 75 filtered out; finished in 0.00s
4. Optionally, in order to use the benchmarking functionality, please install `valgrind` e.g. `sudo apt install valgrind`.

### Solving puzzles
Start implementing solutions!
    - Copy and paste your input for the day (e.g. [2020 Day 1's input](https://adventofcode.com/2020/day/1/input)) into the matching numbered file in the inputs directory
    - Implement the solution in the matching numbered dayXX.rs file in src
        - Run the program using `cargo run` (with the day number to run just that one day, rather than all of 1-25).  Add `--release` to perform a release build for a faster run!
    - (Optional) Add examples from the puzzle statement into tests in the same file.
        - Run the tests using `cargo test` (with the day number to run just the appropriate tests, rather than the tests for every day).

### Benchmarking
Pass `--bench` when running (e.g. `cargo run 0 --bench`) to benchmark your code using [iai](https://github.com/bheisler/iai). For the purposes of benchmarking, each solution is split into `parse_input`, `part_one` and `part_two`. 

## Other things I might at some point add...
- [ ] benchmarking using criterion
- [ ] cargo flamegraph CPU profiles
- [ ] heap allocation info using valgrind/massif
- [ ] better parsing of bench output
