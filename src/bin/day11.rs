use color_eyre::Result;
use itertools::Itertools;
use petgraph::graph::{DiGraph, NodeIndex};
use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

#[derive(Debug, Clone)]
pub struct Input<'a> {
    source: &'a str,
    targets: Vec<&'a str>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day11.txt"))?;
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
    use color_eyre::Result;
    use nom::character::complete::alpha1;
    use nom::error::Error;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Input<'_>> {
        map(
            separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
            |(source, targets)| Input { source, targets },
        )(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input<'_>>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn solve_part1(input: &[Input]) -> u32 {
    let (g, indices) = build_graph(input);
    let target_node = *indices.get("out").unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(*indices.get("you").unwrap());
    let mut result = 0;
    while let Some(node) = queue.pop_front() {
        if node == target_node {
            result += 1;
            continue;
        }
        for n in g.neighbors(node) {
            queue.push_back(n);
        }
    }
    result
}

fn solve_part2(input: &[Input]) -> u32 {
    let (g, indices) = build_graph(input);
    // do graph search but somehow track node sightings
    graph_search(&g, &indices, *indices.get("svr").unwrap(), false, false)
}

fn graph_search<'a>(
    g: &DiGraph<usize, usize, u32>,
    indices: &HashMap<&'a str, NodeIndex>,
    current: NodeIndex,
    seen_dac: bool,
    seen_fft: bool,
) -> u32 {
    let current_name = indices.iter().find(|(_, v)| **v == current).unwrap().0;
    if *current_name == "out" {
        if seen_dac && seen_fft {
            return 1;
        } else {
            return 0;
        }
    }
    let mut seen_dac_real = seen_dac;
    if *current_name == "dac" {
        seen_dac_real = true;
    }
    let mut seen_fft_real = seen_fft;
    if *current_name == "fft" {
        seen_fft_real = true;
    }
    let mut result = 0;
    for n in g.neighbors(current) {
        result += graph_search(g, indices, n, seen_dac_real, seen_fft_real);
    }
    result
}

fn build_graph<'a>(
    input: &'a [Input<'a>],
) -> (DiGraph<usize, usize, u32>, HashMap<&'a str, NodeIndex>) {
    let mut g: DiGraph<usize, usize, u32> = DiGraph::new();
    let indices: HashMap<&str, NodeIndex> = input
        .iter()
        .flat_map(|i| i.targets.iter().chain(std::iter::once(&i.source)))
        .unique()
        .map(|x| (*x, g.add_node(0)))
        .collect();
    g.extend_with_edges(input.iter().flat_map(|i| {
        let s = indices.get(i.source).unwrap();
        i.targets.iter().map(|t| (*s, *indices.get(t).unwrap()))
    }));
    (g, indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day11.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 5);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day11.test.2.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 2);
        Ok(())
    }
}
