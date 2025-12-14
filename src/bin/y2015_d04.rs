// https://adventofcode.com/2015/day/4

use advent_of_code::read_input;

fn part1(input: &str) -> usize {
    let mut buf = itoa::Buffer::new();
    for i in 0.. {
        let mut md5_ctx = md5::Context::new();
        md5_ctx.consume(input.as_bytes());
        md5_ctx.consume(buf.format(i));
        let digest = md5_ctx.finalize();
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
            return i;
        }
    }
    0
}

fn part2(input: &str) -> usize {
    let mut buf = itoa::Buffer::new();
    for i in 0.. {
        let mut md5_ctx = md5::Context::new();
        md5_ctx.consume(input.as_bytes());
        md5_ctx.consume(buf.format(i));
        let digest = md5_ctx.finalize();
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return i;
        }
    }
    0
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
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
