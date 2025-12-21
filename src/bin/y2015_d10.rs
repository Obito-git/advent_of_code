// https://adventofcode.com/2015/day/10

use advent_of_code::read_input;
use itertools::Itertools;

fn next_look_and_say(initial: Vec<u8>) -> Vec<u8> {
    let mut res = Vec::with_capacity(initial.len() * 3 / 2);

    for (digit, val) in &initial.iter().chunk_by(|&x| x) {
        let count = val.count() as u8;
        res.push(count);
        res.push(*digit);
    }
    res
}

fn solve(initial: Vec<u8>, iterations: usize) -> usize {
    let mut cur = initial;
    for _ in 0..iterations {
        cur = next_look_and_say(cur);
    }
    cur.len()
}

fn part1(input: Vec<u8>) -> usize {
    solve(input, 40)
}

fn part2(input: Vec<u8>) -> usize {
    solve(input, 50)
}

fn main() {
    let contents = read_input();
    println!(
        "Part 1: {}",
        part1(contents.chars().map(|c| c as u8 - b'0').collect())
    );
    println!(
        "Part 2: {}",
        part2(contents.chars().map(|c| c as u8 - b'0').collect())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_look_and_say() {
        let seq1 = vec![1];
        assert_eq!(next_look_and_say(seq1), vec![1, 1]);

        let seq2 = vec![1, 1];
        assert_eq!(next_look_and_say(seq2), vec![2, 1]);

        let seq3 = vec![2, 1];
        assert_eq!(next_look_and_say(seq3), vec![1, 2, 1, 1]);

        let seq4 = vec![1, 2, 1, 1];
        assert_eq!(next_look_and_say(seq4), vec![1, 1, 1, 2, 2, 1]);

        let seq5 = vec![1, 1, 1, 2, 2, 1];
        assert_eq!(next_look_and_say(seq5), vec![3, 1, 2, 2, 1, 1]);

        let seq6 = vec![3, 1, 2, 2, 1, 1];
        assert_eq!(next_look_and_say(seq6), vec![1, 3, 1, 1, 2, 2, 2, 1]);

        // 13112221
        let seq7 = vec![1, 3, 1, 1, 2, 2, 2, 1];
        assert_eq!(next_look_and_say(seq7), vec![1, 1, 1, 3, 2, 1, 3, 2, 1, 1]);

        let seq8 = vec![1, 1, 1, 3, 2, 1, 3, 2, 1, 1];
        assert_eq!(
            next_look_and_say(seq8),
            vec![3, 1, 1, 3, 1, 2, 1, 1, 1, 3, 1, 2, 2, 1]
        );
    }
}
