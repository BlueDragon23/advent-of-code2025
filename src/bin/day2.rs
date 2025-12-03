use advent_of_code2025::range::Range;
use color_eyre::Result;
use itertools::Itertools;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day2.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input),
        time.elapsed().as_millis()
    );
    let time = Instant::now();
    println!(
        "Part 2: {} in {}ms",
        solve_part2(&input),
        time.elapsed().as_millis()
    );
    Ok(())
}

mod parsing {

    use advent_of_code2025::parsing::parse_number;
    use advent_of_code2025::range::Range;
    use color_eyre::Result;
    use nom::error::Error;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Vec<Range<u64>>> {
        separated_list1(
            tag(","),
            map(
                separated_pair(parse_number, tag("-"), parse_number),
                |(lower, upper)| Range { lower, upper },
            ),
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Range<u64>>, Error<&str>> {
        input
            .lines()
            .next()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .unwrap()
    }
}

fn solve_part1(input: &Vec<Range<u64>>) -> u64 {
    input.iter().fold(0, |acc, r| {
        acc + (r.lower..=r.upper).into_iter().fold(0, |inner, x| {
            let mut text = x.to_string();
            if text.len() % 2 == 0 {
                let second_half = text.split_off(text.len() / 2);
                if second_half == text {
                    return inner + x;
                }
            }
            inner
        })
    })
}

fn solve_part2(input: &Vec<Range<u64>>) -> u64 {
    input.iter().fold(0, |acc, r| {
        acc + (r.lower..=r.upper).into_iter().fold(0, |inner, x| {
            let text = x.to_string();
            for chunk_size in 1..=(text.len()/2) {
                if text.len() % chunk_size == 0 {
                    if text.chars().chunks(chunk_size).into_iter().map(|chars| chars.collect_vec()).all_equal() {
                        return inner + x
                    }
                }
            }
            inner
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day2.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 1227775554);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day2.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 4174379265);
        Ok(())
    }
}
