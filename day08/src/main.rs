use std::collections::HashMap;
use std::str::FromStr;
use color_eyre::eyre::eyre;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{alphanumeric1, line_ending, space1};
use nom::combinator::{all_consuming, value};
use nom::error::Error;
use nom::{Finish};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use color_eyre::{Report, Result};

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    color_eyre::install()?;
    let network: Network = INPUT.parse()?;
    let steps = network
        .follow_instructions("AAA", "ZZZ")
        .ok_or_else(|| eyre!("Cannot follow steps"))?;
    println!("Day 08 part 1: {steps}");
    let steps = network
        .follow_ghost_instructions("A", "Z")
        .ok_or_else(|| eyre!("Cannot follow steps"))?;
    println!("Day 08 part 2: {steps}");
    Ok(())
}

type Node = String;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Instruction {
    Left,
    Right,
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Neighbour {
    left: Node,
    right: Node,
}


#[derive(Debug, Eq, PartialEq)]
struct Network {
    nodes: HashMap<Node, Neighbour>,
    instructions: Vec<Instruction>,
}

impl Network {
    fn follow_instructions(&self, start: &str, end: &str) -> Option<usize> {
        let mut current = start.to_string();
        for (step, instruction) in self.instructions.iter().cycle().enumerate() {
            if current == end {
                return Some(step);
            }
            current = match instruction {
                Instruction::Left => self.nodes.get(&current)?.left.clone(),
                Instruction::Right => self.nodes.get(&current)?.right.clone(),
            };
        }
        unreachable!()
    }

    fn get_cycle_length(&self, start: &str, end_suffix: &str) -> Option<usize> {
        let mut current = start.to_string();
        for (step, instruction) in self.instructions.iter().cycle().enumerate() {
            if current.ends_with(end_suffix) {
                return Some(step);
            }
            current = match instruction {
                Instruction::Left => self.nodes.get(&current)?.left.clone(),
                Instruction::Right => self.nodes.get(&current)?.right.clone(),
            };
        }
        unreachable!()
    }

    fn follow_ghost_instructions(&self, start_suffix: &str, end_suffix: &str) -> Option<usize> {
        let current: Vec<Node> = self.nodes
            .keys()
            .filter(|key| key.ends_with(start_suffix))
            .cloned()
            .collect();

        let cycles: Vec<usize> = current
            .iter()
            .map(|c| self.get_cycle_length(c, end_suffix))
            .collect::<Option<Vec<usize>>>()?;
        let mut lcm_value = cycles[0];
        for x in cycles.iter().skip(1) {
            lcm_value = lcm(lcm_value, *x);
        }
        Some(lcm_value)
    }
}

impl FromStr for Network {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = alt((
            value(Instruction::Left, tag::<_, _, Error<_>>("L")),
            value(Instruction::Right, tag_no_case("R"))));

        let node = tuple((
            terminated(alphanumeric1, tuple((space1, tag("="), space1))),
            delimited(tag("("), separated_pair(
                alphanumeric1, tuple((tag(","), space1)), alphanumeric1,
            ), tag(")"))
        ));

        let mut parser = all_consuming(
            separated_pair(
                many1(
                    instruction
                ),
                tuple((line_ending, line_ending)),
                separated_list1(
                    line_ending,
                    node,
                ),
            )
        );
        let (instructions, nodes_vec) = parser(s.trim())
            .finish()
            .map_err(|err| eyre!("Cannot parse: {err}"))?.1;
        let mut nodes: HashMap<Node, Neighbour> = HashMap::with_capacity(nodes_vec.len());
        for (root, (left, right)) in nodes_vec {
            nodes.insert(root.to_string(), Neighbour {
                left: left.to_string(),
                right: right.to_string(),
            });
        }
        Ok(Self {
            nodes,
            instructions,
        })
    }
}

const fn gcd(mut a: usize, mut b: usize) -> usize {
    // wikipedia ftw.
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

const fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");

    #[test]
    fn it_parses_input() {
        let network: Network = EXAMPLE.parse().unwrap();
        let mut expected_map: HashMap<Node, Neighbour> = HashMap::new();
        expected_map.insert("AAA".to_string(), Neighbour {
            left: "BBB".to_string(),
            right: "BBB".to_string(),
        });
        expected_map.insert("BBB".to_string(), Neighbour {
            left: "AAA".to_string(),
            right: "ZZZ".to_string(),
        });
        expected_map.insert("ZZZ".to_string(), Neighbour {
            left: "ZZZ".to_string(),
            right: "ZZZ".to_string(),
        });
        assert_eq!(network.instructions, vec![Instruction::Left, Instruction::Left, Instruction::Right]);
        assert_eq!(network.nodes, expected_map);
    }

    #[test]
    fn it_follows_instructions() {
        let network: Network = EXAMPLE.parse().unwrap();
        assert_eq!(network.follow_instructions("AAA", "ZZZ"), Some(6));
    }

    #[test]
    fn it_follows_ghost_instructions() {
        let network: Network = EXAMPLE2.parse().unwrap();
        assert_eq!(network.follow_ghost_instructions("A", "Z"), Some(6));
    }
}
