// https://adventofcode.com/2015/day/12

use advent_of_code::read_input;

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
    let mut stack: Vec<(i32, bool)> = vec![(0, false)]; // (sum, has_red)
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            b'{' => {
                stack.push((0, false));
                i += 1;
            }
            b'}' => {
                let (sum, has_red) = stack.pop().unwrap();
                if !has_red {
                    stack.last_mut().unwrap().0 += sum;
                }
                i += 1;
            }
            b':' => {
                i += 1;
                while i < input.len() && input[i].is_ascii_whitespace() {
                    i += 1;
                }
                if i < input.len() && input[i] == b'"' {
                    if input[i..].starts_with(b"\"red\"") {
                        stack.last_mut().unwrap().1 = true;
                        i += 5;
                        continue;
                    }
                }
            }
            _ if byte_can_be_number(&input[i]) => {
                let start = i;
                while i < input.len() && byte_can_be_number(&input[i]) {
                    i += 1;
                }
                let num: i32 = std::str::from_utf8(&input[start..i])
                    .unwrap()
                    .parse()
                    .unwrap();
                stack.last_mut().unwrap().0 += num;
            }
            _ => i += 1,
        }
    }

    stack[0].0
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
        let input1 = "[1,2,3]";
        assert_eq!(part2(input1.as_bytes()), 6);
    }
    #[test]
    fn test_part2_example2() {
        let input1 = "[1,{\"c\":\"red\",\"b\":2},3]";
        assert_eq!(part2(input1.as_bytes()), 4);
    }
    #[test]
    fn test_part2_example3() {
        let input1 = "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}";
        assert_eq!(part2(input1.as_bytes()), 0);
    }
    #[test]
    fn test_part2_example4() {
        let input1 = "[1,\"red\",5]";
        assert_eq!(part2(input1.as_bytes()), 6);
    }
}
