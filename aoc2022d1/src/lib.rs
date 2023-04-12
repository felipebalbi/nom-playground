use clap::Parser;
use nom::{
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    IResult,
};
use std::cmp::Reverse;

type Calorie = u32;

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d1",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 1",
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

fn parse_calorie(input: &str) -> IResult<&str, Calorie> {
    let (input, calories) = separated_list1(newline, complete::u32)(input)?;
    let calorie = calories.iter().sum();

    Ok((input, calorie))
}

fn parse_calories(input: &str) -> IResult<&str, Vec<Calorie>> {
    separated_list1(many1(newline), parse_calorie)(input)
}

fn part1(input: &str) -> u32 {
    let (_, result) = parse_calories(input).unwrap();

    result.into_iter().max().unwrap()
}

fn part2(input: &str) -> u32 {
    let (_, mut result) = parse_calories(input).unwrap();

    result.sort_by_key(|k| Reverse(*k));

    result.into_iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn part2_works() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(part2(input), 45000);
    }
}
