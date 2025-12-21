// https://adventofcode.com/2015/day/9

use advent_of_code::read_input;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_distance(line: &str) -> (&str, &str, usize) {
    let mut split = line.split(" = ");
    let (from, to) = {
        let mut places = split.next().unwrap().split(" to ");
        (places.next().unwrap(), places.next().unwrap())
    };
    (from, to, split.next().unwrap().parse().unwrap())
}

fn part1(input: &str) -> usize {
    let mut res = usize::MAX;
    let mut distances: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    for line in input.lines() {
        let (from, to, distance) = parse_distance(line);
        distances.entry(from).or_default().insert(to, distance);
        distances.entry(to).or_default().insert(from, distance);
    }

    for perm in distances.keys().permutations(distances.keys().len()) {
        let mut total = 0;
        for window in perm.windows(2) {
            total += distances[window[0]][window[1]]
        }
        res = res.min(total);
    }

    res
}

fn part2(input: &str) -> usize {
    let mut res = usize::MIN;
    let mut distances: HashMap<&str, HashMap<&str, usize>> = HashMap::new();

    for line in input.lines() {
        let (from, to, distance) = parse_distance(line);
        distances.entry(from).or_default().insert(to, distance);
        distances.entry(to).or_default().insert(from, distance);
    }

    for perm in distances.keys().permutations(distances.keys().len()) {
        let mut total = 0;
        for window in perm.windows(2) {
            total += distances[window[0]][window[1]]
        }
        res = res.max(total);
    }

    res
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 982);
    }
}
