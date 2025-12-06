use color_eyre::Result;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
    numbers: Vec<Vec<u64>>,
    operators: Vec<Operator>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Multiply,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day6.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let second_input = parsing::parse_input_2(include_str!("../../input/day6.txt"));
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&second_input),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {

    use crate::Operator;

    use super::Input;
    use advent_of_code2025::parsing::parse_number;
    use color_eyre::Result;
    use itertools::Itertools;
    use nom::branch::alt;
    use nom::error::Error;
    use nom::multi::{many0, many1, separated_list1};
    use nom::sequence::{delimited, preceded, separated_pair};
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            separated_pair(
                separated_list1(
                    tag("\n"),
                    delimited(
                        many0(tag(" ")),
                        separated_list1(many1(tag(" ")), parse_number),
                        many0(tag(" ")),
                    ),
                ),
                tag("\n"),
                separated_list1(
                    many1(tag(" ")),
                    preceded(
                        many0(tag(" ")),
                        alt((
                            map(tag("+"), |_| Operator::Plus),
                            map(tag("*"), |_| Operator::Multiply),
                        )),
                    ),
                ),
            ),
            |(numbers, operators)| Input { numbers, operators },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Input, Error<&str>> {
        parse_line(input).finish().map(|x| x.1)
    }

    pub fn parse_input_2(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.chars().collect_vec()).collect_vec()
    }
}

fn solve_part1(input: &Input) -> u64 {
    (0..input.operators.len())
        .map(|i| match input.operators[i] {
            Operator::Plus => input.numbers.iter().map(|xs| xs[i]).sum::<u64>(),
            Operator::Multiply => input.numbers.iter().map(|xs| xs[i]).product(),
        })
        .sum()
}

fn solve_part2(input: &Vec<Vec<char>>) -> u64 {
    let max_len = input.iter().map(|l| l.len()).max().unwrap();
    let mut current_nums: Vec<u64> = Vec::new();
    let mut result: u64 = 0;
    let mut skip_next = false;
    for i in (0..max_len).rev() {
        if skip_next {
            skip_next = false;
            continue;
        }
        // Working right to left
        // Build the number
        current_nums.push(
            input
                .iter()
                .take(input.len() - 1)
                .map(|l| l.get(i).unwrap_or(&' '))
                .collect::<String>()
                .trim()
                .parse()
                .unwrap(),
        );
        match input.last().unwrap().get(i).unwrap_or(&' ') {
            '*' => {
                result += current_nums.iter().product::<u64>();
                current_nums = Vec::new();
                skip_next = true;
            }
            '+' => {
                result += current_nums.iter().sum::<u64>();
                current_nums = Vec::new();
                skip_next = true;
            }
            _ => {}
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day6.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 4277556);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input_2(include_str!("../../input/day6.test.txt"));
        let result = solve_part2(&input);
        assert_eq!(result, 3263827);
        Ok(())
    }
}
