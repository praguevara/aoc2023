use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
struct Map<'a> {
    directions: &'a [Direction],
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> TryFrom<&'a str> for Map<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();

        let directions: &[_] = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!(),
            })
            .collect::<Vec<_>>()
            .leak();

        let lines = lines.skip(1);
        let re = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").unwrap();

        let nodes = lines
            .map(|line| {
                let caps = re.captures(line).unwrap();
                (
                    caps.get(1).unwrap().as_str(),
                    Node {
                        name: caps.get(1).unwrap().as_str(),
                        left: caps.get(2).unwrap().as_str(),
                        right: caps.get(3).unwrap().as_str(),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Map { directions, nodes })
    }
}

fn steps_to_reach_zzz(map: &Map) -> usize {
    let mut steps = 0;
    let mut current_node = &map.nodes["AAA"];
    let target_node = &map.nodes["ZZZ"];

    let cycled_directions = map.directions.iter().cycle();
    for next_direction in cycled_directions {
        current_node = &map.nodes[match next_direction {
            Direction::Left => current_node.left,
            Direction::Right => current_node.right,
        }];
        steps += 1;

        if current_node == target_node {
            break;
        }
    }

    steps
}

fn steps_to_end_at_xxz(map: &Map, node_name: &str) -> usize {
    let mut steps = 0;
    let mut current_node = &map.nodes[node_name];
    let cycled_directions = map.directions.iter().cycle();
    for next_direction in cycled_directions {
        current_node = &map.nodes[match next_direction {
            Direction::Left => current_node.left,
            Direction::Right => current_node.right,
        }];

        steps += 1;

        if current_node.name.as_bytes()[2] == b'Z' {
            return steps;
        }
    }
    unreachable!()
}

fn steps_to_reach_all_xxz(map: &Map) -> usize {
    map.nodes
        .iter()
        .filter(|n| n.0.as_bytes()[2] == b'A')
        .map(|(_, v)| v)
        .map(|node| steps_to_end_at_xxz(map, node.name))
        .reduce(num::integer::lcm)
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    let map = Map::try_from(input).unwrap();
    // let steps = steps_to_reach_zzz(&map);
    let steps = steps_to_reach_all_xxz(&map);
    println!("{}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        include_str!("../sample.txt")
    }

    fn sample2() -> &'static str {
        include_str!("../sample2.txt")
    }

    fn sample3() -> &'static str {
        include_str!("../sample3.txt")
    }

    #[test]
    fn test_parse() {
        let map: Map = sample().try_into().unwrap();
        assert_eq!(map.directions.len(), 2);
        assert_eq!(map.nodes.len(), 7);

        assert_eq!(map.nodes["AAA"].left, "BBB");
        assert_eq!(map.nodes["AAA"].right, "CCC");
    }

    #[test]
    fn test_num_steps() {
        let map_1 = sample().try_into().unwrap();
        let steps_1 = steps_to_reach_zzz(&map_1);
        assert_eq!(steps_1, 2);

        let map2 = sample2().try_into().unwrap();
        let steps_2 = steps_to_reach_zzz(&map2);
        assert_eq!(steps_2, 6);
    }

    #[test]
    fn test_num_steps_all_xxz() {
        let map = sample3().try_into().unwrap();
        let steps = steps_to_reach_all_xxz(&map);
        assert_eq!(steps, 6);
    }
}
