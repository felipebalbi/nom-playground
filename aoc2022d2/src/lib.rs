use clap::Parser;
use nom::{
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d2",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 2",
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

#[derive(Debug, Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(item: char) -> Self {
        match item {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

impl From<Shape> for u32 {
    fn from(item: Shape) -> Self {
        match item {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Strategy {
    Win,
    Lose,
    Draw,
}

impl From<char> for Strategy {
    fn from(item: char) -> Self {
        match item {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl From<Strategy> for u32 {
    fn from(item: Strategy) -> Self {
        match item {
            Strategy::Win => 6,
            Strategy::Draw => 3,
            Strategy::Lose => 0,
        }
    }
}

fn parse_shape(input: &str) -> IResult<&str, Shape> {
    map(complete::one_of("ABCXYZ"), |c| c.into())(input)
}

fn parse_strategy(input: &str) -> IResult<&str, Strategy> {
    map(complete::one_of("XYZ"), |c| c.into())(input)
}

fn parse_line_part1(input: &str) -> IResult<&str, (Shape, Shape)> {
    separated_pair(parse_shape, complete::char(' '), parse_shape)(input)
}

fn parse_line_part2(input: &str) -> IResult<&str, (Shape, Strategy)> {
    separated_pair(parse_shape, complete::char(' '), parse_strategy)(input)
}

fn parse_input_part1(input: &str) -> IResult<&str, Vec<(Shape, Shape)>> {
    separated_list1(newline, parse_line_part1)(input)
}

fn parse_input_part2(input: &str) -> IResult<&str, Vec<(Shape, Strategy)>> {
    separated_list1(newline, parse_line_part2)(input)
}

fn part1(input: &str) -> u32 {
    let (_, games) = parse_input_part1(input).unwrap();
    let mut score: u32 = 0;

    for game in games {
        let opponent = game.0;
        let player = game.1;

        let result = match opponent {
            Shape::Rock => match player {
                Shape::Paper => Strategy::Win,
                Shape::Rock => Strategy::Draw,
                Shape::Scissors => Strategy::Lose,
            },
            Shape::Paper => match player {
                Shape::Paper => Strategy::Draw,
                Shape::Rock => Strategy::Lose,
                Shape::Scissors => Strategy::Win,
            },
            Shape::Scissors => match player {
                Shape::Paper => Strategy::Lose,
                Shape::Rock => Strategy::Win,
                Shape::Scissors => Strategy::Draw,
            },
        };

        let shape_value: u32 = player.clone().into();
        let result_value: u32 = result.clone().into();

        score += shape_value;
        score += result_value;
    }

    score
}

fn part2(input: &str) -> u32 {
    let (_, games) = parse_input_part2(input).unwrap();
    let mut score: u32 = 0;

    for game in games {
        let shape = game.0;
        let strategy = game.1;

        let play = match shape {
            Shape::Rock => match strategy {
                Strategy::Win => Shape::Paper,
                Strategy::Draw => Shape::Rock,
                Strategy::Lose => Shape::Scissors,
            },
            Shape::Paper => match strategy {
                Strategy::Win => Shape::Scissors,
                Strategy::Draw => Shape::Paper,
                Strategy::Lose => Shape::Rock,
            },
            Shape::Scissors => match strategy {
                Strategy::Win => Shape::Rock,
                Strategy::Draw => Shape::Scissors,
                Strategy::Lose => Shape::Paper,
            },
        };

        let shape_value: u32 = play.clone().into();
        let strategy_value: u32 = strategy.clone().into();

        score += shape_value;
        score += strategy_value;
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "A Y
B X
C Z";

        assert_eq!(part1(input), 15);
    }

    #[test]
    fn part2_works() {
        let input = "A Y
B X
C Z";

        assert_eq!(part2(input), 12);
    }
}
