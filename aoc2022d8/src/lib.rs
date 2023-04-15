use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, line_ending, not_line_ending, one_of, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d8",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 8",
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

#[derive(Debug, PartialOrd, PartialEq)]
struct Tree {
    height: u32,
}

fn tree(input: &str) -> IResult<&str, Tree> {
    map(one_of("0123456789"), |c| Tree {
        height: c.to_digit(10).unwrap(),
    })(input)
}

fn row(input: &str) -> IResult<&str, Vec<Tree>> {
    many1(tree)(input)
}

fn rows(input: &str) -> IResult<&str, Vec<Vec<Tree>>> {
    separated_list1(line_ending, row)(input)
}

fn parse_input_part1(input: &str) -> IResult<&str, Vec<Vec<Tree>>> {
    rows(input)
}

fn parse_input_part2(input: &str) -> IResult<&str, Vec<&str>> {
    todo!()
}

fn part1(input: &str) -> u32 {
    let (_, trees) = parse_input_part1(input).unwrap();
    let mut visible = vec![];

    for (i, row) in trees.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            if i == 0 || i == trees.len() - 1 {
                visible.push(true);
                continue;
            }

            if j == 0 || j == row.len() - 1 {
                visible.push(true);
                continue;
            }

            if taller_than_neighbors(&t, i, j, &trees) {
                visible.push(true);
                continue;
            }

            visible.push(false);
        }
    }

    dbg!(visible.len());
    dbg!(visible.iter().filter(|v| **v).count());

    420
}

/// this is wrong. I need to check that there is a path from the
/// current tree to any of the four edges. Perhaps a good time to
/// learn petgraph?
fn taller_than_neighbors(t: &Tree, i: usize, j: usize, trees: &Vec<Vec<Tree>>) -> bool {
    let north = &trees[i - 1][j];
    let south = &trees[i + 1][j];
    let east = &trees[i][j + 1];
    let west = &trees[i][j - 1];

    t > north || t > south || t > east || t > west
}

fn part2(input: &str) -> u32 {
    420
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(part1(input), 21);
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let input = "30373
25512
65332
33549
35390";

        assert_eq!(part2(input), 42);
    }
}
