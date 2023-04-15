use std::collections::BTreeMap;

use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, line_ending, not_line_ending, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d7",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 7",
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

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Inode<'a>>),
}

#[derive(Debug)]
enum Inode<'a> {
    Dir(&'a str),
    File { name: &'a str, size: u32 },
}

fn file(input: &str) -> IResult<&str, Inode> {
    map(
        separated_pair(complete::u32, space1, is_a("abcdefghijklmnopqrstuvwxyz.")),
        |(size, name)| Inode::File { name, size },
    )(input)
}

fn directory(input: &str) -> IResult<&str, Inode> {
    map(separated_pair(tag("dir"), space1, alpha1), |(_, name)| {
        Inode::Dir(name)
    })(input)
}

fn inodes(input: &str) -> IResult<&str, Vec<Inode>> {
    separated_list1(line_ending, alt((file, directory)))(input)
}

fn ls(input: &str) -> IResult<&str, Command> {
    map(
        tuple((terminated(tag("$ ls"), line_ending), inodes)),
        |(_, inodes)| Command::Ls(inodes),
    )(input)
}

fn cd(input: &str) -> IResult<&str, Command> {
    map(
        separated_pair(tag("$ cd"), space1, not_line_ending),
        |(_, name)| match name {
            "/" => Command::Cd(Cd::Root),
            ".." => Command::Cd(Cd::Up),
            _ => Command::Cd(Cd::Down(name)),
        },
    )(input)
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(line_ending, alt((ls, cd)))(input)
}

fn parse_input_part1(input: &str) -> IResult<&str, Vec<Command>> {
    commands(input)
}

fn parse_input_part2(input: &str) -> IResult<&str, ()> {
    todo!()
}

fn fold_sizes<'a>(
    (mut stack, mut table): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    cmd: &'a Command,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match cmd {
        Command::Cd(Cd::Root) => {
            stack.push("");
        }
        Command::Cd(Cd::Up) => {
            stack.pop();
        }
        Command::Cd(Cd::Down(name)) => {
            stack.push(name);
        }
        Command::Ls(inodes) => {
            let size = inodes
                .iter()
                .filter_map(|inode| {
                    if let Inode::File { size, .. } = inode {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>();

            for i in 0..stack.len() {
                table
                    .entry(stack[0..=i].to_vec())
                    .and_modify(|v| *v += size)
                    .or_insert(size);
            }
        }
    };
    (stack, table)
}

fn part1(input: &str) -> u32 {
    let (_, cmds) = parse_input_part1(input).unwrap();

    let (_, table) = cmds
        .iter()
        .fold((Vec::default(), BTreeMap::default()), fold_sizes);

    table
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u32>()
}

fn part2(input: &str) -> usize {
    420
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

        assert_eq!(part1(input), 95437);
    }

    #[test]
    fn part2_works() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

        assert_eq!(part2(input), 42);
    }
}
