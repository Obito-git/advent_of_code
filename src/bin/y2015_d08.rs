// https://adventofcode.com/2015/day/8

use advent_of_code::read_input;
use std::str::Chars;

fn count_chars(mut iter: Chars) -> usize {
    let mut count = 0;
    while let Some(next_char) = iter.next() {
        if next_char == '"' {
            continue;
        }
        if next_char == '\\' {
            let next_plus_one = iter.next();
            if matches!(next_plus_one, Some('x')) {
                iter.next();
                iter.next();
            }
        }
        count += 1;
    }
    count
}

fn part1(input: &str) -> usize {
    let (l, c) = input.lines().fold((0, 0), |(len, c), s| {
        (len + s.len(), c + count_chars(s.chars()))
    });
    l - c
}

fn count_encoded(mut initial_len: usize, mut iter: Chars) -> usize {
    while let Some(next_char) = iter.next() {
        if next_char == '"' {
            initial_len += 2;
            continue;
        }
        if next_char == '\\' {
            initial_len += 2;
            let next_plus_one = iter.next();
            if matches!(next_plus_one, Some('x')) {
                iter.next();
                iter.next();
                initial_len -= 1;
            }
        }
    }
    initial_len
}

fn part2(input: &str) -> usize {
    let (init, enc) = input.lines().fold((0, 0), |(len, enc), s| {
        (len + s.len(), enc + count_encoded(s.len(), s.chars()))
    });
    enc - init
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 19);
    }
}
