use clap::Parser;
use std::collections::BTreeSet;

#[derive(Parser, Debug)]
#[command(
    name = "aoc2022d6",
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Advent of Code 2022 Day 6",
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

fn find_packet_of_length(input: &str, length: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(length)
        .enumerate()
        .filter(|(_, window)| {
            let set = window.iter().collect::<BTreeSet<_>>();
            set.len() == window.len()
        })
        .collect::<Vec<_>>()
        .first()
        .map(|(i, _)| i + length)
        .unwrap_or(0)
}

fn part1(input: &str) -> usize {
    find_packet_of_length(input, 4)
}

fn part2(input: &str) -> usize {
    find_packet_of_length(input, 14)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part1(input), 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part1(input), 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part1(input), 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn part2_works() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part2(input), 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part2(input), 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part2(input), 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part2(input), 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part2(input), 26);
    }
}
