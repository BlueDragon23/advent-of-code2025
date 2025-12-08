use advent_of_code2025::range::Range;
use color_eyre::Result;
use itertools::Itertools;
use num::abs;
use std::{
    cmp::{max, min},
    time::Instant,
};

#[derive(Debug, Clone, Copy)]
pub struct Input {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day9.txt"))?;
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
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            separated_pair(parse_number, tag(","), parse_number),
            |(x, y)| Input { x, y },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &[Input]) -> i64 {
    input
        .iter()
        .cartesian_product(input.iter())
        .map(|(a, b)| (abs(b.x - a.x) + 1) * (abs(b.y - a.y) + 1))
        .sorted()
        .rev()
        .max()
        .unwrap()
}

fn solve_part2(input: &[Input]) -> i64 {
    let horizontal_boundaries = input
        .iter()
        .chain(vec![input[0]].iter())
        .tuple_windows()
        .filter_map(|(a, b)| {
            if a.x == b.x {
                None
            } else {
                Some((
                    a.y,
                    Range {
                        lower: min(a.x, b.x),
                        upper: max(a.x, b.x),
                    },
                ))
            }
        })
        .collect_vec();
    let vertical_boundaries = input
        .iter()
        .chain(vec![input[0]].iter())
        .tuple_windows()
        .filter_map(|(a, b)| {
            if a.y == b.y {
                None
            } else {
                Some((
                    a.x,
                    Range {
                        lower: min(a.y, b.y),
                        upper: max(a.y, b.y),
                    },
                ))
            }
        })
        .collect_vec();

    let (corner_a, corner_b, size) = input
        .iter()
        .cartesian_product(input.iter())
        .map(|(a, b)| (a, b, (abs(b.x - a.x) + 1) * (abs(b.y - a.y) + 1)))
        .sorted_by_key(|x| x.2)
        .rev()
        .find(|(a, b, _)| {
            // println!("{:?}, {:?}", a, b);
            // Filter for a valid rectangle, starting from the largest
            // Walk the edges, checking to see if we cross a boundary
            validate_boundary_crossing(&horizontal_boundaries, a, &Input { x: a.x, y: b.y })
                && validate_boundary_crossing(&horizontal_boundaries, b, &Input { x: b.x, y: a.y })
                && validate_boundary_crossing(&vertical_boundaries, a, &Input { x: b.x, y: a.y })
                && validate_boundary_crossing(&vertical_boundaries, &b, &Input { x: a.x, y: b.y })
        })
        .unwrap();
    println!("{:?}, {:?}, {}", corner_a, corner_b, size);
    size
}

fn validate_boundary_crossing(
    boundaries: &Vec<(i64, Range<i64>)>,
    starting_point: &Input,
    end_point: &Input,
) -> bool {
    let stable;
    let dynamic_start;
    let dynamic_end;
    if starting_point.x == end_point.x {
        stable = starting_point.x;
        dynamic_start = starting_point.y;
        dynamic_end = end_point.y;
    } else {
        stable = starting_point.y;
        dynamic_start = starting_point.x;
        dynamic_end = end_point.x;
    };

    for dynamic in min(dynamic_start, dynamic_end)..=max(dynamic_start, dynamic_end) {
        // Don't need to check our starting point, since it's guaranteed good
        if dynamic == dynamic_start || dynamic == dynamic_end {
            continue;
        }
        if boundaries
            .iter()
            .find(|(boundary_dynamic, r)| *boundary_dynamic == dynamic && r.contains(stable))
            .is_some()
        {
            // println!("Found boundary at {},{}", stable, dynamic);
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day9.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 50);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day9.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 24);
        Ok(())
    }
}
