#[macro_use]
extern crate lazy_static;
use clap::{App, Arg};
use std::time::Instant;
use std::fs;

mod _2023;
mod utils;

fn main() {
    let matches = App::new("RuS2m's solutions for Advent of Code")
        .arg(Arg::with_name("year").required(true))
        .arg(Arg::with_name("day").required(true))
        .get_matches();

    let year = matches.value_of("year").unwrap();
    let day = matches.value_of("day").unwrap();
    let formatted_day = format!("{:02}", day.parse::<u32>().expect("Invalid day format"));
    let filename = format!("resources/{}-{}.txt", year, formatted_day);

    let input = fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    match (year, day) {
        ("2023", "1") => run_solution(_2023::day1::solve, &input),
        ("2023", "2") => run_solution(_2023::day2::solve, &input),
        ("2023", "3") => run_solution(_2023::day3::solve, &input),
         _ => eprintln!("Invalid year/day combination"),
    }
}

fn run_solution<F, E>(solve_fn: F, input: &Vec<String>)
where
    F: Fn(Vec<String>) -> Result<(Result<String, E>, Result<String, E>), E>,
    E: std::fmt::Display + std::fmt::Debug,
{
    let start = Instant::now();
    let solution = solve_fn(input.clone());
    let duration = start.elapsed();

    match solution {
        Ok((part1, part2)) => {
            print_result("Answer for the Part 1", part1);
            print_result("Answer for the Part 2", part2);
        }
        Err(e) => eprintln!("Error running solution: {}", e),
    }

    println!("Time elapsed: {:?}", duration);
}

fn print_result(part: &str, result: Result<String, impl std::fmt::Display>) {
    match result {
        Ok(val) => println!("{}: {}", part, val),
        Err(e) => println!("{}: Error - {}", part, e),
    }
}
