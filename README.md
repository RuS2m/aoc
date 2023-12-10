# 🎄 Advent of Code Solutions [![bin](https://github.com/RuS2m/aoc/actions/workflows/all.yml/badge.svg)](https://github.com/RuS2m/aoc/actions/workflows/all.yml) ![GitHub last commit](https://img.shields.io/github/last-commit/RuS2m/aoc)

Welcome to my repository of solutions for the [Advent Of Code](https://adventofcode.com) challenges, written in Rust! This is both my first year participating in Advent of Code and my first month exploring Rust, so this journey is as much about learning the language as it is about solving the puzzles.

> [!NOTE]
> As a beginner in Rust, the code here may not reflect the best practices or most efficient implementations. However, this is part of my learning process, and I expect the quality of the code to improve over time.

## Project Structure and Usage
- **Solutions Directory:** Each year's solutions are placed in their respective directories under `src`. For instance, *2023* solutions are in `src/_2023`.
- **Daily Solutions:** Individual solution files are named according to the day, e.g., `day3.rs` for *Day 3*'s solution.
- **Input Files:** Input data for each day's challenge is in the `resources` folder, named as `YYYY-DD.txt` (e.g., `resources/2023-03.txt` for *Day 3 of 2023*).

### Running Solutions
Run solutions using Cargo by specifying the year and day:

```bash
cargo run -- <year> <day>
```

For example, to execute the solution for *Day 3 of 2023*:
```bash
cargo run -- 2023 3
```

### Testing

Each solution has associated tests based on provided examples. Run all tests with:

```bash
cargo test
```

## Solutions

### [2023](https://adventofcode.com/2023) [![2023](https://github.com/RuS2m/aoc/actions/workflows/2023_tests.yml/badge.svg)](https://github.com/RuS2m/aoc/actions/workflows/2023_tests.yml)
- [Day 1: Trebuchet?!](src/_2023/day1.rs)
- [Day 2: Cube Conundrum](src/_2023/day2.rs)
- [Day 3: Gear Ratios](src/_2023/day3.rs)
- [Day 4: Scratchcards](src/_2023/day4.rs)
- [Day 5: If You Give A Seed A Fertilizer](src/_2023/day5.rs)
- [Day 6: Wait For It](src/_2023/day6.rs)
- [Day 7: Camel Cards](src/_2023/day7.rs)
