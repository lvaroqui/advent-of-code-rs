# Advent of Code - Rust

My solutions to (some) days of [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org) 🦀.

# Framework

This repository provides facilities to make *Advent of Code* more enjoyable:
 - Automatically generate the boilerplate for a new day 🖊️
 - Automatically download inputs 🌐
 - Run one or more solutions, with builtin time measurements ⌚
 - Solution labeling to provide multiple versions for the same day 🏷️
 - Run solutions agains unit tests ⚗️

# How to use ?


## Prerequisites

Inputs are automatically downloaded from *AoC* website, you should provide your session key in a [solvers/session-key](./solvers/session-key) file.

## Running solutions

The wrapper script [cargo.sh](./cargo.sh) ensures the glue code is generated. You should use it instead of directly calling `cargo`.

The solutions are run by the [cli](./solvers/cli/), here are a few example like so:

```bash
# Run day 2 of 2024
./cargo.sh run -- --year 2024 --day 2

# Run day 7 of 2024 labelled 'iterative'
./cargo.sh run --release -- -y 2024 -d 7 -l iterative

# Run all solutions from year 2024
./cargo.sh run --release -- -y 2024 -l all

# Run all solutions
./cargo.sh run --release -- -y 2024

# Run day 17 from 2024 against their tests
./cargo.sh run --release -- -y 2024
```

## Tests

Tests are defined in the [solvers/tests](./solvers/tests/) folder, a test folder for a specific day should look like this:
```
tests/
  <year>/
    <day>/
      input_1
      input_2 (optional, input_1 is used for part 2 if absent)
      answer_1
      answer_2
```

# TODOs

- Check solution against AoC automatically(with cache to avoid spamming the site)?
- Import [2021](https://github.com/lvaroqui/advent-of-code-2021-rust) and remove repo from github