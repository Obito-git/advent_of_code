// https://adventofcode.com/2015/day/1

use advent_of_code::read_input;

fn part1(s: &str) -> i32 {
    s.chars().fold(0, |acc, c| {
        match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc,
        }
    })
}

fn part2(s: &str) -> usize {
    s.chars().scan(0, |floor, c| {
        *floor += if c == '(' { 1 } else { -1 };
        Some(*floor)
    }).position(|floor| floor == -1).unwrap() + 1
}

fn main() {
    let contents = read_input();
    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_part1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }
}
