use aoc2022d7::Cli;
use clap::Parser;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    Cli::parse().run(&input);
}
