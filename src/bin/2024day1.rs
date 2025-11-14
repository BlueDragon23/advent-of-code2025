use color_eyre::Result;
use num::abs;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone)]
pub struct Input {
    left: i32,
    right: i32
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/2024day1.txt"))?;
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
    use color_eyre::Result;
    use nom::error::Error;
    use nom::sequence::separated_pair;
    use nom::{bytes::complete::tag, combinator::map, Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(separated_pair(parse_number, tag("   "), parse_number), |(left, right)| Input {left, right})(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &[Input]) -> i32 {
    let (mut ls, mut rs) = input.iter().fold((Vec::new(), Vec::new()), |(mut lefts, mut rights), i| {
        lefts.push(i.left);
        rights.push(i.right);
        (lefts, rights)
    });
    ls.sort();
    rs.sort();
    ls.iter().zip(rs).map(|(l, r)| abs(l - r)).sum()
}

fn solve_part2(input: &[Input]) -> i32 {
    let (ls, rs): (HashMap<i32, i32>, HashMap<i32, i32>) = input.iter().fold((HashMap::new(), HashMap::new()), |(mut lefts, mut rights), i| {
        *lefts.entry(i.left).or_default() += 1;
        *rights.entry(i.right).or_default() += 1;
        (lefts, rights)
    });
    ls.iter().fold(0, |acc, (l, reps)| {
        acc + l * reps * rs.get(l).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/2024day1.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/2024day1.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 31);
        Ok(())
    }
}
