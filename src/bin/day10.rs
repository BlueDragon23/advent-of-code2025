use color_eyre::Result;
use itertools::{Itertools, repeat_n};
use petgraph::{
    algo::dijkstra,
    graph::{DiGraph, NodeIndex},
};
use std::{collections::{HashMap, VecDeque}, time::Instant};

#[derive(Debug, Clone)]
pub struct Input {
    // Use for bits
    target_state: Vec<bool>,
    buttons: Vec<Vec<bool>>,
    joltage: Vec<u32>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    let input = parsing::parse_input(include_str!("../../input/day10.txt"))?;
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
    use itertools::Itertools;
    use nom::branch::alt;
    use nom::character::complete::space1;
    use nom::error::Error;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, tuple};
    use nom::{Finish, IResult, bytes::complete::tag, combinator::map};

    fn parse_line(input: &str) -> IResult<&str, Input> {
        map(
            tuple((
                delimited(
                    tag("["),
                    many1(map(alt((tag("."), tag("#"))), |c| match c {
                        "." => false,
                        "#" => true,
                        _ => panic!("Nope"),
                    })),
                    tag("]"),
                ),
                space1,
                separated_list1(
                    tag(" "),
                    delimited(tag("("), separated_list1(tag(","), parse_number), tag(")")),
                ),
                space1,
                delimited(tag("{"), separated_list1(tag(","), parse_number), tag("}")),
            )),
            |(target_state, _, buttons, _, joltage): (_, _, Vec<Vec<usize>>, _, _)| {
                let state_size = target_state.len();
                Input {
                    target_state,
                    buttons: buttons
                        .iter()
                        .map(|bs| {
                            vec![false; state_size]
                                .iter()
                                .enumerate()
                                .map(|(i, _)| bs.contains(&i))
                                .collect_vec()
                        })
                        .collect_vec(),
                    joltage,
                }
            },
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
    let mut result = 0;
    for i in input {
        let size = i.target_state.len();
        let mut g = DiGraph::new();
        let all_states: HashMap<Vec<bool>, NodeIndex> = repeat_n(vec![false, true], size)
            .multi_cartesian_product()
            .map(|s| {
                let index = g.add_node(s.clone());
                (s, index)
            })
            .collect();
        // Instead of proper search, just traverse every possible state
        // There's only like 1024, it's fine-ish even if some are impossible to get to
        for (possible_state, state_index) in all_states.iter() {
            for button in &i.buttons {
                let new_state = possible_state
                    .clone()
                    .iter()
                    .enumerate()
                    .map(|(i, b)| b ^ button[i])
                    .collect_vec();
                g.add_edge(*state_index, *all_states.get(&new_state).unwrap(), 1);
            }
        }
        // Now find the shortest path from start to end
        let source_index = *all_states.get(&vec![false; size]).unwrap();
        let target_index = *all_states.get(&i.target_state).unwrap();
        let costs = dijkstra(&g, source_index, Some(target_index), |_| 1);
        result += costs.get(&target_index).unwrap();
    }
    result
}

fn solve_part2(input: &[Input]) -> u32 {
    let mut result = 0;
    for (row, i) in input.iter().enumerate() {
        println!("{}", row);
        let size = i.joltage.len();
        let mut g = DiGraph::new();
        let mut all_states: HashMap<Vec<u32>, NodeIndex> = HashMap::new();
        // We have to do proper search now :(
        let mut to_visit = VecDeque::new();
        let initial = vec![0; size];
        let initial_index = g.add_node(initial.clone());
        to_visit.push_back((initial, initial_index));
        // TODO: it would probably be faster to do DFS, trying the button presses with the most digits first
        // since if we need to have N total changes, doing more at once is better
        // and again we could stop once we reach it, since the first search reaching there should be optimal
        while let Some((state, state_index)) = to_visit.pop_front() {
            // since we're doing BFS, as soon as we reach the target state we can stop since it must be the shortest path
            // We could skip the graph search by storing our depth
            if state == i.joltage {
                break;
            }
            for button in &i.buttons {
                let new_state = state.clone().iter().enumerate().map(|(i, b)| {
                    if button[i] {
                        b + 1
                    } else {
                        *b
                    }
                }).collect_vec();
                if new_state.iter().enumerate().any(|(index, value)| *value > i.joltage[index]) {
                    // If we've gone over the target, give up
                    continue;
                }
                let target_index = all_states.entry(new_state.clone()).or_insert_with(|| {
                    let i = g.add_node(new_state.clone());
                    to_visit.push_back((new_state.clone(), i));
                    i
                });
                g.add_edge(state_index, *target_index, 1);
            }
        }
        // Now find the shortest path from start to end
        let target_index = *all_states.get(&i.joltage).unwrap();
        let costs = dijkstra(&g, initial_index, Some(target_index), |_| 1);
        result += costs.get(&target_index).unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day10.test.txt"))?;
        let result = solve_part1(&input);
        assert_eq!(result, 7);
        Ok(())
    }

    #[test]
    fn test_part2() -> color_eyre::Result<()> {
        let input = parsing::parse_input(include_str!("../../input/day10.test.txt"))?;
        let result = solve_part2(&input);
        assert_eq!(result, 33);
        Ok(())
    }
}
