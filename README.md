# Advent of Code

Rust solutions for [Advent of Code](https://adventofcode.com/).

## Creating a New Challenge

Run the helper script to scaffold a new day:

```bash
./new_day.sh
```

The script will prompt you for the year and day, then create:
- A Rust source file with a basic template (`src/bin/y{year}_d{day}.rs`)
- An empty input file (`inputs/y{year}_d{day}.txt`)

After creation, paste your puzzle input into the generated `.txt` file.

## Running Solutions

```bash
echo "inputs/y2015_d01.txt" | cargo run --bin y2015_d01
```

## Running Tests

```bash
cargo test --bin y2015_d01
```

## Structure

- `src/lib.rs` - shared helpers
- `src/bin/y{year}_d{day}.rs` - solutions
- `inputs/y{year}_d{day}.txt` - puzzle inputs
- `new_day.sh` - script to create new challenge files
