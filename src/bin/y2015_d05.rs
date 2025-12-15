// https://adventofcode.com/2015/day/5

use advent_of_code::read_input;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn is_nice(line: &str) -> bool {
    let line_bytes = line.as_bytes();
    let mut vowels_count = 0;
    let mut contains_twice = false;
    for i in 0..line.len() {
        if let (Some(&a), Some(&b)) = (line_bytes.get(i), line_bytes.get(i + 1)) {
            match (a, b) {
                (b'a', b'b') | (b'c', b'd') | (b'p', b'q') | (b'x', b'y') => return false,
                _ if a == b => contains_twice = true,
                _ => {}
            }
        }
        if matches!(line_bytes[i] as char, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowels_count += 1
        }
    }
    vowels_count >= 3 && contains_twice
}

fn part1(input: &str) -> usize {
    input.lines().map(is_nice).filter(|v| *v).count()
}

fn is_nice2(line: &str) -> bool {
    let line_bytes = line.as_bytes();
    let mut twice_map: HashMap<(u8, u8), usize> = HashMap::new();
    let mut contains_twice = false;
    let mut contains_between = false;
    for i in 0..line.len() {
        if let (Some(&a), Some(&b)) = (line_bytes.get(i), line_bytes.get(i + 1)) {
            match twice_map.entry((a, b)) {
                Entry::Occupied(e) if i >= *e.get() + 2 => contains_twice = true,
                Entry::Vacant(e) => {
                    e.insert(i);
                }
                _ => {}
            };
        }
        if let (Some(&a), Some(&c)) = (line_bytes.get(i), line_bytes.get(i + 2))
            && a == c
        {
            contains_between = true
        }
        if contains_twice && contains_between {
            return true;
        }
    }
    contains_twice && contains_between
}

fn part2(input: &str) -> usize {
    input.lines().map(is_nice2).filter(|v| *v).count()
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
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part2() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
