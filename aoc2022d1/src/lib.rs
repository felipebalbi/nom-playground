use nom::{
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    IResult,
};
use std::cmp::Reverse;

type Calorie = u32;

fn parse_calorie(input: &str) -> IResult<&str, Calorie> {
    let (input, calories) = separated_list1(newline, complete::u32)(input)?;
    let calorie = calories.iter().sum();

    Ok((input, calorie))
}

fn parse_calories(input: &str) -> IResult<&str, Vec<Calorie>> {
    separated_list1(many1(newline), parse_calorie)(input)
}

pub fn part1(input: &str) -> u32 {
    parse_calories(input).unwrap().1.into_iter().max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut result = parse_calories(input).unwrap().1;

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
