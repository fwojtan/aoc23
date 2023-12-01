use advent_of_code_template::{bench_day, solve_day};
use clap::Parser;

#[derive(Parser)]
#[command(author="Finlay Wojtan", version="0.1.0", about="Advent of Code test and benchmarking template", long_about = None)]
struct Cli {
    /// Selects a single day to run. If not specified, all days are run.
    day: Option<i32>,

    /// Benchmarks the solution for given days.
    #[arg(short, long)]
    bench: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut days = (0..=25).collect::<Vec<i32>>();
    let mut days_to_execute = vec![];
    if let Some(day) = cli.day {
        if !days.contains(&day) {
            panic!("Day not found");
        }
        days_to_execute.push(day);
    } else {
        days_to_execute = days.drain(1..).collect(); // Skip day0 example
    }
    for day in days_to_execute {
        if cli.bench {
            bench_day(&day);
        } else {
            solve_day(&day, true);
        }
    }
}
