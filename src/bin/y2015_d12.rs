// https://adventofcode.com/2015/day/12

use advent_of_code::read_input;
use std::collections::HashMap;

fn can_be_number(c: &char) -> bool {
    c.is_numeric() || *c == '-'
}

fn byte_can_be_number(b: &u8) -> bool {
    (*b as char).is_numeric() || *b as char == '-'
}

fn part1(input: &str) -> i32 {
    let mut res: i32 = 0;
    let mut it = input.chars().peekable();
    while it.peek().is_some() {
        let _ = it.by_ref().skip_while(|c| !can_be_number(c));
        let n_str: String = it.by_ref().take_while(can_be_number).collect();
        if n_str.is_empty() {
            continue;
        }
        res += n_str.parse::<i32>().unwrap();
    }
    res
}

fn part2(input: &[u8]) -> i32 {
    let mut curly_level = 0;
    let mut res = 0;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            b'{' => {
                curly_level += 1;
                i += 1
            }
            b'r' if matches!(input.get(i + 1), Some(b'e'))
                && matches!(input.get(i + 2), Some(b'd')) => {}
            b'}' => {}
            _ if byte_can_be_number(&input[i]) => {
                let num: String = input[i..]
                    .iter()
                    .copied()
                    .take_while(byte_can_be_number)
                    .map(|b| b as char)
                    .collect();
                map.entry(curly_level)
                    .or_default()
                    .push(num.parse().unwrap());
                i += num.len()
            }
            _ => i += 1,
        }
    }
    todo!()
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(contents.as_bytes()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input1 = "[1,2,3]";
        let input2 = "{\"a\":2,\"b\":4}";
        assert_eq!(part1(input1), 6);
        assert_eq!(part1(input2), 6);
    }

    #[test]
    fn test_part1_example2() {
        let input1 = "[[[3]]]";
        let input2 = "{\"a\":{\"b\":4},\"c\":-1}";
        assert_eq!(part1(input1), 3);
        assert_eq!(part1(input2), 3);
    }

    #[test]
    fn test_part1_example3() {
        let input1 = "{\"a\":[-1,1]}";
        let input2 = "[-1,{\"a\":1}]";
        assert_eq!(part1(input1), 0);
        assert_eq!(part1(input2), 0);
    }

    #[test]
    fn test_part1_example4() {
        let input1 = "{}";
        let input2 = "[]";
        assert_eq!(part1(input1), 0);
        assert_eq!(part1(input2), 0);
    }

    #[test]
    fn test_part2_example1() {
        let input1 = "[144,2,3]";
        assert_eq!(part2(input1.as_bytes()), 6);
    }
}
