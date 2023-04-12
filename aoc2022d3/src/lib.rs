use clap::Parser;
use nom::{
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list1,
    IResult,
};

type Priority = i32;
type Type = char;
type Item = (Priority, Type);
type Compartment = Vec<Item>;
type Rucksack = (Compartment, Compartment);

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d3",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 3",
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

fn char_to_priority(c: char) -> i32 {
    if c.is_uppercase() {
        (c as i32) - ('A' as i32)
    } else {
        (c as i32) - ('a' as i32)
    }
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    map(
        complete::one_of("abcdefghjklmnopqrstuvwxyzABCDEFGHJKLMNOPQRSTUVWXYZ"),
        |c| (char_to_priority(c), c),
    )(input)
}

fn parse_rucksack(input: &str) -> IResult<&str, Rucksack> {
    todo!()
}

fn parse_input_part1(input: &str) -> IResult<&str, Vec<Rucksack>> {
    todo!()
}

fn parse_input_part2(input: &str) -> IResult<&str, Vec<Rucksack>> {
    todo!()
}

fn part1(input: &str) -> u32 {
    42
}

fn part2(input: &str) -> u32 {
    42
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part1(input), 157);
    }

    #[test]
    fn part2_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part2(input), 42);
    }
}
