// https://adventofcode.com/2015/day/7

use advent_of_code::read_input;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct WireIdentifier(u8, u8);

impl From<&str> for WireIdentifier {
    fn from(value: &str) -> Self {
        let raw = value.as_bytes();
        WireIdentifier(raw[0], raw.get(1).copied().unwrap_or(0))
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Wire(WireIdentifier),
    Value(u16),
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        if let Ok(val) = s.parse::<u16>() {
            Operand::Value(val)
        } else {
            Operand::Wire(WireIdentifier::from(s))
        }
    }
}

enum Gate {
    Assign(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, u16),
    Rshift(Operand, u16),
    Not(Operand),
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        let mut tokens = value.split_whitespace();
        let len = tokens.clone().count();

        match len {
            1 => Gate::Assign(Operand::from(tokens.next().unwrap())),
            2 => {
                if tokens.next().unwrap() != "NOT" {
                    panic!("invalid input")
                }
                Gate::Not(Operand::from(tokens.next().unwrap()))
            }
            3 => {
                let left = Operand::from(tokens.next().unwrap());
                let op = tokens.next().unwrap();
                let right_str = tokens.next().unwrap();

                match op {
                    "AND" => Gate::And(left, Operand::from(right_str)),
                    "OR" => Gate::Or(left, Operand::from(right_str)),
                    "LSHIFT" => Gate::Lshift(left, right_str.parse().unwrap()),
                    "RSHIFT" => Gate::Rshift(left, right_str.parse().unwrap()),
                    _ => panic!("invalid input: {}", op),
                }
            }
            _ => panic!("invalid input: {}", value),
        }
    }
}

fn parse_line(line: &str) -> (WireIdentifier, Gate) {
    let mut operands = line.split(" -> ");
    let raw_gate = operands.next().unwrap();
    let wire = {
        let raw = operands.next().unwrap();
        WireIdentifier::from(raw)
    };
    (wire, Gate::from(raw_gate))
}

fn resolve_operand(
    map: &HashMap<WireIdentifier, Gate>,
    visited: &mut HashMap<WireIdentifier, u16>,
    op: &Operand,
) -> u16 {
    match op {
        Operand::Value(v) => *v,
        Operand::Wire(id) => find_signal(map, visited, id),
    }
}

fn find_signal(
    map: &HashMap<WireIdentifier, Gate>,
    visited: &mut HashMap<WireIdentifier, u16>,
    id: &WireIdentifier,
) -> u16 {
    if let Some(&val) = visited.get(id) {
        return val;
    }

    let gate = map.get(id).expect(&format!("wire {:?} not found", id));

    let res = match gate {
        Gate::Assign(op) => resolve_operand(map, visited, op),
        Gate::And(op1, op2) => {
            resolve_operand(map, visited, op1) & resolve_operand(map, visited, op2)
        }
        Gate::Or(op1, op2) => {
            resolve_operand(map, visited, op1) | resolve_operand(map, visited, op2)
        }
        Gate::Lshift(op, shift) => resolve_operand(map, visited, op) << shift,
        Gate::Rshift(op, shift) => resolve_operand(map, visited, op) >> shift,
        Gate::Not(op) => !resolve_operand(map, visited, op),
    };

    visited.insert(*id, res);
    res
}

fn part1(input: &str, id: WireIdentifier) -> u16 {
    let map = input
        .lines()
        .map(parse_line)
        .collect::<HashMap<WireIdentifier, Gate>>();
    let mut visited = HashMap::new();
    find_signal(&map, &mut visited, &id)
}

fn part2(input: &str, id: WireIdentifier) -> u16 {
    let map = input
        .lines()
        .map(parse_line)
        .collect::<HashMap<WireIdentifier, Gate>>();
    let mut visited = HashMap::new();
    let a_value = find_signal(&map, &mut visited, &id);
    visited.clear();
    visited.insert(WireIdentifier(b'b', 0), a_value);
    find_signal(&map, &mut visited, &id)
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(&contents, WireIdentifier(b'a', 0)));
    println!("Part 2: {}", part2(&contents, WireIdentifier(b'a', 0)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, WireIdentifier(b'd', 0)), 72);
        assert_eq!(part1(INPUT, WireIdentifier(b'e', 0)), 507);
        assert_eq!(part1(INPUT, WireIdentifier(b'f', 0)), 492);
        assert_eq!(part1(INPUT, WireIdentifier(b'g', 0)), 114);
        assert_eq!(part1(INPUT, WireIdentifier(b'h', 0)), 65412);
        assert_eq!(part1(INPUT, WireIdentifier(b'i', 0)), 65079);
        assert_eq!(part1(INPUT, WireIdentifier(b'x', 0)), 123);
        assert_eq!(part1(INPUT, WireIdentifier(b'y', 0)), 456);
    }
}
