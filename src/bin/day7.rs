use color_eyre::Result;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[derive(Debug, Clone)]
pub struct Input {
    row: Vec<Node>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Node {
    Start,
    Empty,
    Splitter,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day7.txt"))?;
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

    use crate::Node;

    use super::Input;
    use color_eyre::Result;
    use nom::character::complete::anychar;
    use nom::error::Error;
    use nom::multi::many1;
    use nom::{Finish, IResult, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            many1(map(anychar, |c| match c {
                '.' => Node::Empty,
                'S' => Node::Start,
                '^' => Node::Splitter,
                _ => panic!("Illegal"),
            })),
            |row| Input { row },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &[Input]) -> u32 {
    let pos = input[0]
        .row
        .iter()
        .find_position(|n| **n == Node::Start)
        .unwrap()
        .0;
    let mut rays: HashSet<usize> = HashSet::new();
    rays.insert(pos);
    let mut result = 0;
    for r in input.iter().skip(1) {
        let mut next_rays = HashSet::new();
        let row = &r.row;
        for (index, node) in row.iter().enumerate() {
            if *node == Node::Splitter && rays.contains(&index) {
                // Split
                next_rays.insert(index - 1);
                next_rays.insert(index + 1);
                result += 1;
            } else if rays.contains(&index) {
                next_rays.insert(index);
            }
        }
        rays = next_rays;
    }
    result
}

fn solve_part2(input: &[Input]) -> u64 {
    let pos = input[0]
        .row
        .iter()
        .find_position(|n| **n == Node::Start)
        .unwrap()
        .0;
    let mut rays: HashMap<usize, u64> = HashMap::new();
    rays.insert(pos, 1);
    for r in input.iter().skip(1) {
        let mut next_rays = HashMap::new();
        let row = &r.row;
        for (index, node) in row.iter().enumerate() {
            if *node == Node::Splitter && rays.contains_key(&index) {
                // Split
                *next_rays.entry(index - 1).or_default() += rays.get(&index).unwrap();
                *next_rays.entry(index + 1).or_default() += rays.get(&index).unwrap();
            } else if rays.contains_key(&index) {
                *next_rays.entry(index).or_default() += rays.get(&index).unwrap();
            }
        }
        rays = next_rays;
    }
    rays.values().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day7.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 21);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day7.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 40);
        Ok(())
    }
}
