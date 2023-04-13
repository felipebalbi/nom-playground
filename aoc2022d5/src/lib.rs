use clap::Parser;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{self, alpha1, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d5",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 5",
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
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    map(
        alt((delimited(tag("["), alpha1, tag("]")), tag("   "))),
        |krate| match krate {
            "   " => None,
            k => Some(k),
        },
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), parse_crate)(input)
}

fn transpose_crates(crates: Vec<Vec<Option<&str>>>) -> Vec<Vec<&str>> {
    let len = crates[0].len();
    let mut iters: Vec<_> = crates.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .filter(|n| n.is_some())
                .map(|n| n.unwrap())
                .rev()
                .collect::<Vec<_>>()
        })
        .collect()
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (input, crates) = separated_list1(line_ending, parse_line)(input)?;
    let crates = transpose_crates(crates);
    Ok((input, crates))
}

fn parse_crate_id(input: &str) -> IResult<&str, ()> {
    value((), pair(take_until("\n\n"), tag("\n\n")))(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            tag("move "),
            complete::u32,
            tag(" from "),
            complete::u32,
            tag(" to "),
            complete::u32,
        )),
        |(_, amount, _, from, _, to)| Move {
            amount: amount as usize,
            from: (from - 1) as usize,
            to: (to - 1) as usize,
        },
    )(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(line_ending, parse_move)(input)
}

fn parse_input_part1(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates) = parse_crates(input)?;
    let (input, _) = parse_crate_id(input)?;
    let (input, moves) = parse_moves(input)?;

    Ok((input, (crates, moves)))
}

fn parse_input_part2(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    parse_input_part1(input)
}

fn part1(input: &str) -> String {
    let (_, (mut crates, moves)) = parse_input_part1(input).unwrap();

    for m in moves {
        for _ in 0..m.amount {
            if let Some(krate) = crates[m.from].pop() {
                crates[m.to].push(krate);
            }
        }
    }

    let msg = crates
        .iter()
        .map(|krate| krate.last().unwrap().clone())
        .collect::<Vec<_>>();

    msg.join("")
}

fn part2(input: &str) -> String {
    let (_, (mut crates, moves)) = parse_input_part2(input).unwrap();

    for m in moves {
        let len = crates[m.from].len();
        let tail = crates[m.from].split_off(len - m.amount);
        crates[m.to].extend(tail);
    }

    let msg = crates
        .iter()
        .map(|krate| krate.last().unwrap().clone())
        .collect::<Vec<_>>();

    msg.join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

        assert_eq!(part1(input), "CMZ");
    }

    #[test]
    fn part2_works() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

        assert_eq!(part2(input), "MCD");
    }
}
