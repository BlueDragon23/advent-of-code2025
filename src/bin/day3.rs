use color_eyre::Result;
use num::pow;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day3.txt"))?;
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

    use color_eyre::Result;
    use nom::bytes::complete::take;
    use nom::combinator::map_res;
    use nom::error::Error;
    use nom::multi::many1;
    use nom::{Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Vec<u128>> {
        many1(map_res(take(1u8), |c| u128::from_str_radix(c, 10)))(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Vec<u128>>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &Vec<Vec<u128>>) -> u128 {
    // find largest two digit number
    input
        .iter()
        .map(|bank| {
            // don't include the last digit
            let (max_tens_pos, max_tens_val) = bank.iter().take(bank.len() - 1).enumerate().fold(
                (0, 0),
                |(max_pos, max_val), (pos, val)| {
                    if *val > max_val {
                        (pos, *val)
                    } else {
                        (max_pos, max_val)
                    }
                },
            );
            // don't include anything before n
            let (_, max_ones_val) = bank.iter().skip(max_tens_pos + 1).enumerate().fold(
                (0, 0),
                |(max_pos, max_val), (pos, val)| {
                    if *val > max_val {
                        (pos, *val)
                    } else {
                        (max_pos, max_val)
                    }
                },
            );
            max_tens_val * 10 + max_ones_val
        })
        .sum()
}

fn solve_part2(input: &Vec<Vec<u128>>) -> u128 {
    // find largest twelve digit number
    input
        .iter()
        .map(|bank| {
            let mut start_index = 0;
            let mut result = 0;
            for i in (0..12).rev() {
                let (max_pos, max_val) = bank
                    .iter()
                    .enumerate()
                    .skip(start_index)
                    .take(bank.len() - start_index - i)
                    .fold((0, 0), |(max_pos, max_val), (pos, val)| {
                        if *val > max_val {
                            (pos, *val)
                        } else {
                            (max_pos, max_val)
                        }
                    });
                start_index = max_pos + 1;
                result += max_val * pow(10, i);
            }
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day3.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 357);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day3.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 3121910778619);
        Ok(())
    }
}
