// https://adventofcode.com/2015/day/6

use advent_of_code::read_input;
use std::str::SplitWhitespace;

//TODO: try without grid materialization

const TURN_ON: u8 = 1;
const TURN_OFF: u8 = 2;
const TOGGLE: u8 = 3;

fn to_coord(value: &str) -> (u8, (usize, usize), (usize, usize)) {
    let (command, rest) = if let Some(rest) = value.strip_prefix("turn on ") {
        (TURN_ON, rest)
    } else if let Some(rest) = value.strip_prefix("turn off ") {
        (TURN_OFF, rest)
    } else {
        (TOGGLE, value.strip_prefix("toggle ").unwrap())
    };

    let mut parts = rest.split_whitespace();
    let take_coord = |parts: &mut SplitWhitespace| {
        let coords = parts.next().unwrap();
        let (x, y) = coords.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    };

    let start = take_coord(&mut parts);
    parts.next();
    let end = take_coord(&mut parts);

    (command, start, end)
}

fn part1(input: &str) -> i32 {
    let lines = input.lines();
    let mut grid = vec![vec![false; 1000]; 1000];
    for line in lines {
        let (com, (start_x, start_y), (end_x, end_y)) = to_coord(line);
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let cell = &mut grid[y][x];
                match com {
                    TURN_ON => *cell = true,
                    TURN_OFF => *cell = false,
                    TOGGLE => *cell = !*cell,
                    _ => {}
                }
            }
        }
    }
    grid.into_iter()
        .flatten()
        .map(|v| if v { 1 } else { 0 })
        .sum()
}

fn part2(input: &str) -> usize {
    let lines = input.lines();
    let mut grid = vec![vec![0usize; 1000]; 1000];
    for line in lines {
        let (com, (start_x, start_y), (end_x, end_y)) = to_coord(line);
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let cell = &mut grid[y][x];
                match com {
                    TURN_ON => *cell += 1,
                    TURN_OFF => *cell = cell.saturating_sub(1),
                    TOGGLE => *cell += 2,
                    _ => {}
                }
            }
        }
    }
    grid.into_iter().flatten().sum()
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}
