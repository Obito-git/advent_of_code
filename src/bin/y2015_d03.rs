// https://adventofcode.com/2015/day/3

use advent_of_code::read_input;
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut visited = HashSet::from([(0, 0)]);
    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => unreachable!(),
        }
        visited.insert((x, y));
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let (mut x1, mut y1) = (0, 0);
    let (mut x2, mut y2) = (0, 0);
    let mut visited = HashSet::from([(0, 0)]);

    for (i, c) in input.chars().enumerate() {
        let (x, y) = if i % 2 == 0 {
            (&mut x1, &mut y1)
        } else {
            (&mut x2, &mut y2)
        };
        match c {
            '^' => *y += 1,
            'v' => *y -= 1,
            '<' => *x -= 1,
            '>' => *x += 1,
            _ => unreachable!(),
        }
        visited.insert((*x, *y));
    }
    visited.len()
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
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
