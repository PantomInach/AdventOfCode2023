use num::Integer;
use std::{collections::HashMap, ops::ControlFlow, str::Lines};

struct Graph {
    nodes: HashMap<String, Node>,
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Graph {
    fn from_lines(lines: Lines) -> Self {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        lines.for_each(|l| {
            let (name, tuple) = l.split_once(" = ").unwrap();
            let (left, right) = tuple[1..tuple.len() - 1].split_once(", ").unwrap();
            nodes.insert(
                name.to_string(),
                Node {
                    name: name.to_string(),
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        });
        Graph { nodes }
    }

    fn find_min_cycle_len(&self, node: &Node, instructions: &str) -> u64 {
        let mut i: u64 = 0;
        instructions.chars().cycle().try_fold(node, |acc, c| {
            i += 1;
            let new_node = match c {
                'L' => self.nodes.get(&acc.left).unwrap(),
                'R' => self.nodes.get(&acc.right).unwrap(),
                _ => unimplemented!("Not possible."),
            };
            if new_node.name.ends_with('Z') {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(new_node)
            }
        });
        i
    }
}

pub fn process_part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();
    let g = Graph::from_lines(lines);

    let mut i: u64 = 0;
    let mut position: &Node = g.nodes.get(&"AAA".to_string()).unwrap();
    let end_node: &Node = g.nodes.get(&"ZZZ".to_string()).unwrap();
    while position != end_node {
        instructions.chars().for_each(|c| {
            i += 1;
            position = match c {
                'L' => g.nodes.get(&position.left).unwrap(),
                'R' => g.nodes.get(&position.right).unwrap(),
                _ => unimplemented!("Not possible."),
            }
        });
    }
    i
}

pub fn process_part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();
    let g = Graph::from_lines(lines);

    // Every start node emits a path to a node ending with Z. This defines a cycle.
    // Due to the hiden design of the input set, it is sufficient to just find a common point where
    // all cycles converge. This is excatly the lowest common multiple of all cycles and the length
    // of the instruction set.
    g.nodes
        .iter()
        .filter(|(name, _)| name.ends_with('A'))
        .map(|(_, node)| g.find_min_cycle_len(node, instructions))
        .fold(instructions.len() as u64, |acc, l| acc.lcm(&l))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_v2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6_u64, process_part2(input));
    }
}
