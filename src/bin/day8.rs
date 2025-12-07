use color_eyre::Result;
use itertools::Itertools;
use num::integer::sqrt;
use petgraph::{
    Graph, graph::UnGraph, visit::{Dfs, EdgeRef}
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    time::Instant,
};

#[derive(Debug, Clone)]
pub struct Input {
    x: i64,
    y: i64,
    z: i64,
}

impl Input {
    pub fn distance(&self, other: &Input) -> i64 {
        sqrt((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2))
    }
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day8.txt"))?;
    let time = Instant::now();
    println!(
        "Part 1: {} in {}ms",
        solve_part1(&input, 1000),
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
    use nom::multi::separated_list1;
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(separated_list1(tag(","), parse_number), |numbers| Input {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2],
        })(input)
    }

    pub fn parse_input(input: &str) -> Result<Vec<Input>, Error<&str>> {
        input
            .lines()
            .map(|line| parse_line(line).finish().map(|x| x.1))
            .collect()
    }
}

fn create_graph(input: &[Input]) -> UnGraph<usize, i64, usize> {
    let edges = input
        .iter()
        .enumerate()
        .flat_map(|(index, junction)| {
            input
                .iter()
                .enumerate()
                .filter_map(move |(other_index, other)| {
                    if index < other_index {
                        Some((index, other_index, junction.distance(other)))
                    } else {
                        None
                    }
                })
        })
        .collect_vec();
    UnGraph::from_edges(edges)
}

fn solve_part1(input: &[Input], connections: usize) -> i64 {
    let g = create_graph(input);

    // Copy the graph to get the same nodes in place
    let mut circuits: UnGraph<usize, i64, usize> = g.clone();
    circuits.clear_edges();
    let closest_elements = g
        .edge_references()
        .sorted_by_key(|e| e.weight())
        .collect_vec();
    for i in 0..connections {
        // smallest distance
        let edge = closest_elements[i];
        // Track nodes joined in new graph
        circuits.add_edge(edge.source(), edge.target(), *edge.weight());
    }
    // find connected components
    let mut to_visit = circuits
        .node_indices()
        .map(|n| (n, false))
        .collect::<HashMap<_, _>>();
    let mut component_sizes = vec![];
    for n in circuits.node_indices() {
        if to_visit[&n] {
            continue;
        }
        let mut component_size = 0;
        let mut dfs = Dfs::new(&circuits, n);
        while let Some(c) = dfs.next(&circuits) {
            to_visit.entry(c).insert_entry(true);
            component_size += 1;
        }
        component_sizes.push(component_size);
    }
    component_sizes.iter().sorted().rev().take(3).product()
}

fn solve_part2(input: &[Input]) -> i64 {
    let g = create_graph(input);

    let closest_elements = g
        .edge_references()
        .sorted_by_key(|e| e.weight())
        .collect_vec();
    let mut components: Vec<RefCell<HashSet<usize>>> = vec![];
    for edge in closest_elements {
        // Four cases.
        // 1. Two new nodes
        // 2. One existing node, one new
        // 3. Both existing nodes in the same component
        // 4. Both existing nodes in different components
        let source = components
            .iter()
            .find_position(|set| set.borrow().contains(&edge.source().index()));
        let target = components
            .iter()
            .find_position(|set| set.borrow().contains(&edge.target().index()));
        if source.is_none() && target.is_none() {
            let mut new_component = HashSet::new();
            new_component.insert(edge.source().index());
            new_component.insert(edge.target().index());
            components.push(RefCell::new(new_component));
        } else if source.is_none() {
            // Now get target mutably
            target.unwrap().1.borrow_mut().insert(edge.source().index());
        } else if target.is_none() {
            source.unwrap().1.borrow_mut().insert(edge.target().index());
        } else if source == target {
            // Do nothing
        } else {
            // source != target, requires merging
            source
                .unwrap()
                .1
                .borrow_mut()
                .extend(target.unwrap().1.borrow().iter());
            components.remove(target.unwrap().0);
        }
        if components.len() == 1 && components[0].borrow().len() == g.node_count() {
            return input[edge.source().index()].x * input[edge.target().index()].x;
        }
    }
    // if this happens, something broke
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day8.test.txt"))?;
        let result = solve_part1(&input, 10);
        assert_eq!(result, 40);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day8.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 25272);
        Ok(())
    }
}
