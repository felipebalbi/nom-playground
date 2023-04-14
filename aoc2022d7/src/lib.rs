use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::{self, alpha1, line_ending, not_line_ending, space1},
    combinator::{map, opt, value},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated, tuple},
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
enum Command<'a> {
    Cd(&'a str),
    Ls,
}

#[derive(Debug)]
enum Inode<'a> {
    Dir(&'a str),
    File { name: &'a str, size: u32 },
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    map(
        tuple((tag("$ "), alpha1, space1, opt(not_line_ending), line_ending)),
        |(_, cmd, _, args, _)| match cmd {
            "cd" => Command::Cd(args.unwrap()),
            "ls" => Command::Ls,
            _ => unreachable!(),
        },
    )(input)
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

fn parse_ls_output_line(input: &str) -> IResult<&str, Inode> {
    terminated(alt((file, directory)), line_ending)(input)
}

fn parse_input_part1(input: &str) -> IResult<&str, ()> {
    dbg!(parse_ls_output_line("dir d\n")?);
    dbg!(parse_ls_output_line("14848518 b.txt\n")?);

    todo!()
}

fn parse_input_part2(input: &str) -> IResult<&str, ()> {
    todo!()
}

fn part1(input: &str) -> usize {
    parse_input_part1(input).unwrap();

    420
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

        assert_eq!(part1(input), 42);
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
