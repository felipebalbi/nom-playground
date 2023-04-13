use clap::Parser;
use nom::{
    character::complete::{self, line_ending},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::ops::RangeInclusive;

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d4",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 4",
    version = "0.1.0"
)]
pub struct Cli {
    #[arg(short = '1', long = "part1", help = "Run part 1.")]
    part1: bool,

    #[arg(short = '2', long = "part2", help = "Run part 2.")]
    part2: bool,
}

impl Cli {
    pub fn run(&self, input: &str) {
        if self.part1 {
            println!("Part 1: {}", part1(input));
        }

        if self.part2 {
            println!("Part 2: {}", part2(input));
        }
    }
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(
        separated_pair(complete::u32, complete::char('-'), complete::u32),
        |(a, b)| a..=b,
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    separated_pair(parse_range, complete::char(','), parse_range)(input)
}

fn parse_input_part1(
    input: &str,
) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    many1(terminated(parse_line, line_ending))(input)
}

fn parse_input_part2(
    input: &str,
) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    parse_input_part1(input)
}

fn fully_contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn part1(input: &str) -> usize {
    let (_, ranges) = parse_input_part1(input).unwrap();

    ranges
        .iter()
        .filter(|(a, b)| fully_contains(&a, &b) || fully_contains(&b, &a))
        .count()
}

fn overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn part2(input: &str) -> usize {
    let (_, ranges) = parse_input_part2(input).unwrap();

    ranges
        .iter()
        .filter(|(a, b)| overlaps(&a, &b) || overlaps(&b, &a))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part2_works() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

        assert_eq!(part2(input), 4);
    }
}
