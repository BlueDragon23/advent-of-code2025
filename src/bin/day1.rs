use color_eyre::Result;
use num::{Integer, abs};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Input {
distance: i32
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day1.txt"))?;
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
    use nom::bytes::complete::take;
    use nom::error::Error;
    use nom::sequence::pair;
    use nom::{combinator::map, Finish, IResult};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(pair(take(1 as u32), parse_number::<i32>), |(direction, distance)| Input {distance: if direction == "L" {
            -distance
        } else {
            distance
        }})(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &[Input]) -> u32 {
    let mut result = 0;
    input.iter().fold(50, |mut acc, i| {
        acc = (acc + i.distance) % 100;
        if acc == 0 {
            result += 1;
        }
        acc
    });
    result
}

fn solve_part2(input: &[Input]) -> u32 {
    let mut result: u32 = 0;
    input.iter().fold(50, |mut acc: i32, i| {
        for _ in 0..abs(i.distance) {
            if i.distance < 0 {
                acc = (acc - 1).rem_euclid(100)
            } else {
                acc = (acc + 1).rem_euclid(100)
            }
            if acc == 0 {
                result += 1
            }
        }
        acc
        // let new_position = (acc + i.distance).rem_euclid(100);
        // if (acc + i.distance) != new_position {
        //     result += 1
        // } else if new_position == 0 {
        //     result += 1
        // }
        // if abs(i.distance) >= 100 {
        //     result += (abs(i.distance).div_floor(&100) as u32) - 1;
        // }
        // new_position
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day1.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day1.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 6);
        Ok(())
    }
}
