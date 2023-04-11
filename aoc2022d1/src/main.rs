use aoc2022d1::{part1, part2};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
