use advent_of_code2025::coordinate::Coordinate;
use color_eyre::Result;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Item {
    EMPTY,
    PAPER,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day4.txt"))?;
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

    use super::Item;

    use color_eyre::Result;
    use nom::branch::alt;
    use nom::error::Error;
    use nom::multi::many1;
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Vec<Item>> {
        many1(alt((
            map(tag("@"), |_| Item::PAPER),
            map(tag("."), |_| Item::EMPTY),
        )))(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Vec<Item>>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &[Vec<Item>]) -> u32 {
    let max_row = input.len();
    let max_col = input[0].len();
    let mut result = 0;
    for row in 0..max_row {
        for col in 0..max_col {
            let coord = Coordinate::new(row, col);
            if coord.get(input) == Item::PAPER {
                let mut count = 0;
                for adj in coord.get_adjacent_points_diagonal(max_row, max_col) {
                    if adj.get(input) == Item::PAPER {
                        count += 1;
                    }
                }
                if count < 4 {
                    result += 1;
                }
            }
        }
    }
    result
}

fn solve_part2(input: &[Vec<Item>]) -> u32 {
    let max_row = input.len();
    let max_col = input[0].len();
    let mut result = 0;
    let mut prev_result;
    let mut current_grid: Vec<Vec<Item>> = input.to_vec();
    loop {
        prev_result = result;
        for row in 0..max_row {
            for col in 0..max_col {
                let coord = Coordinate::new(row, col);
                if coord.get(&current_grid) == Item::PAPER {
                    let mut count = 0;
                    for adj in coord.get_adjacent_points_diagonal(max_row, max_col) {
                        if adj.get(&current_grid) == Item::PAPER {
                            count += 1;
                        }
                    }
                    if count < 4 {
                        result += 1;
                        current_grid[row][col] = Item::EMPTY;
                    }
                }
            }
        }
        if result == prev_result {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day4.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day4.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 43);
        Ok(())
    }
}
