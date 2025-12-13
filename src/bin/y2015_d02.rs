// https://adventofcode.com/2015/day/2

use advent_of_code::read_input;

fn dimensions_from_input(input: &str) -> impl Iterator<Item = (u32, u32, u32)> + use<'_> {
    input.lines().map(|line| {
        let mut dim = line.split('x');
        let l = dim.next().unwrap().parse().unwrap();
        let w = dim.next().unwrap().parse().unwrap();
        let h = dim.next().unwrap().parse().unwrap();
        (l, w, h)
    })
}

fn part1(input: &str) -> u32 {
    dimensions_from_input(input).fold(0, |acc, (l, w, h)| {
        let area1 = l * w;
        let area2 = w * h;
        let area3 = h * l;
        let smallest = area1.min(area2).min(area3);
        acc + 2 * area1 + 2 * area2 + 2 * area3 + smallest
    })
}

fn part2(input: &str) -> u32 {
    dimensions_from_input(input).fold(0, |acc, (l, w, h)| {
        let bow = l * w * h;
        let mut slice = [l, w, h];
        slice.sort();
        acc + bow + slice[0] * 2 + slice[1] * 2
    })
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
        let input1 = "2x3x4";
        let input2 = "1x1x10";

        assert_eq!(part1(input1), 58);
        assert_eq!(part1(input2), 43);
    }

    #[test]
    fn test_part2() {
        let input1 = "2x3x4";
        let input2 = "1x1x10";

        assert_eq!(part2(input1), 34);
        assert_eq!(part2(input2), 14);
    }
}
