use core::panic;
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

fn main() {
    let input = include_str!("../input.txt");
    let map = Map::try_from(input).unwrap();
    let steps = steps_to_reach_zzz(&map);
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

    #[test]
    fn test_parse2() {
        let input = sample();
        let map = Map::try_from(input).unwrap();
        assert_eq!(map.directions.len(), 2);
        assert_eq!(map.nodes.len(), 7);

        assert_eq!(map.nodes["AAA"].left, "BBB");
        assert_eq!(map.nodes["AAA"].right, "CCC");
    }

    #[test]
    fn test_num_steps() {
        let input_1 = sample();
        let map_1 = Map::try_from(input_1).unwrap();
        let steps_1 = steps_to_reach_zzz(&map_1);
        assert_eq!(steps_1, 2);

        let input_2 = sample2();
        let map2 = Map::try_from(input_2).unwrap();
        let steps_2 = steps_to_reach_zzz(&map2);
        assert_eq!(steps_2, 6);
    }
}
