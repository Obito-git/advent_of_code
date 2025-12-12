#!/bin/bash

# Script to create a new Advent of Code challenge

cd "$(dirname "$0")"

read -p "Year (e.g., 2015): " year
read -p "Day (1-25): " day

# Pad day with zero if needed
day_padded=$(printf "%02d" "$day")

name="y${year}_d${day_padded}"
src_file="src/bin/${name}.rs"
input_file="inputs/${name}.txt"

# Check if files already exist
if [[ -f "$src_file" ]]; then
    echo "Error: $src_file already exists!"
    exit 1
fi

# Create directories if they don't exist
mkdir -p src/bin inputs

# Create the Rust source file from template
# First line with variable substitution, rest is literal
echo "// https://adventofcode.com/${year}/day/${day}" > "$src_file"
cat >> "$src_file" << 'EOF'

use advent_of_code::read_input;

fn part1(input: &str) -> i32 {
    todo!()
}

fn part2(input: &str) -> i32 {
    todo!()
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
EOF

# Create empty input file
touch "$input_file"

echo ""
echo "Created:"
echo "  - $src_file"
echo "  - $input_file"
echo ""
echo "Run with:"
echo "  echo \"$input_file\" | cargo run --bin $name"
echo ""
echo "Or run tests with:"
echo "  cargo test --bin $name"
