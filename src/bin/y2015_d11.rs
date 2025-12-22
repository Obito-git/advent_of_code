// https://adventofcode.com/2015/day/11

use advent_of_code::read_input;
use itertools::Itertools;

fn matches_requirements(pass: &[u8]) -> bool {
    let has_three_increasing = pass
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    let double_count = pass
        .windows(2)
        .step_by(1)
        .filter(|w| w[0] == w[1])
        .unique()
        .count();
    // my input doesn't have i,o,l so I can ignore it as I skip it in the calling fn
    // otherwise it will not work
    has_three_increasing && double_count >= 2
}

fn step(input: &mut Vec<u8>, pos: usize) {
    const IGNORED_1: u8 = b'i' - 1;
    const IGNORED_2: u8 = b'o' - 1;
    const IGNORED_3: u8 = b'l' - 1;

    match input[pos] {
        b'z' => {
            input[pos] = b'a';
            if pos == 0 {
                return;
            }
            step(input, pos - 1)
        }
        IGNORED_1 | IGNORED_2 | IGNORED_3 => input[pos] += 2, // skip i, o, l
        _ => input[pos] += 1,
    }
}

fn set_next_password(mut input: &mut Vec<u8>) {
    loop {
        step(&mut input, 7);
        if matches_requirements(&input) {
            return;
        }
    }
}

fn part1(mut input: Vec<u8>) -> String {
    set_next_password(&mut input);
    String::from_utf8(input).unwrap()
}

fn part2(mut input: Vec<u8>) -> String {
    set_next_password(&mut input);
    set_next_password(&mut input);
    String::from_utf8(input).unwrap()
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(contents.as_bytes().to_vec()));
    println!("Part 2: {}", part2(contents.as_bytes().to_vec()));
}
