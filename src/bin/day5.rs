use advent_of_code2025::range::Range;
use color_eyre::Result;
use itertools::Itertools;
use std::{time::Instant};

#[derive(Debug, Clone)]
pub struct Input {
    fresh_ranges: Vec<Range<u64>>,
    ingredients: Vec<u64>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day5.txt"))?;
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

    use super::Input;
    use advent_of_code2025::parsing::parse_number;
    use advent_of_code2025::range::Range;
    use color_eyre::Result;
    use nom::error::Error;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            separated_pair(
                separated_list1(
                    tag("\n"),
                    map(
                        separated_pair(parse_number, tag("-"), parse_number),
                        |(lower, upper)| Range { lower, upper },
                    ),
                ),
                tag("\n\n"),
                separated_list1(tag("\n"), parse_number),
            ),
            |(fresh_ranges, ingredients)| Input {
                fresh_ranges,
                ingredients,
            },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Input, Error<&str>> {
        return parse_line(input).finish().map(|x| x.1);
    }
}

fn solve_part1(input: &Input) -> u32 {
    let mut result = 0;
    for i in &input.ingredients {
        if input.fresh_ranges.iter().find(|r| r.contains(*i)).is_some() {
            result += 1;
        }
    }
    result
}

fn solve_part2(input: &Input) -> u64 {
    input.fresh_ranges.iter().sorted_by_key(|r| r.lower).fold(Vec::new(), |mut acc: Vec<Range<u64>>, fresh| {
        if acc.last().map(|last| last.overlap_or_adjacent(fresh)).unwrap_or_else(|| false) {
            let merged = acc.last().unwrap().merge(fresh);
            let size = acc.len();
            acc[size - 1] = merged;
        } else {
            acc.push(*fresh);
        }
        acc
    }).iter().map(|r| {
        r.upper - r.lower + 1
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day5.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day5.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 14);
        Ok(())
    }
}
