use std::collections::HashMap;

use crate::utils::{self, lcm};

const INPUT: &str = include_str!("input/2023-08.txt");

pub fn run() -> String {
    let map = parse_input(INPUT);
    utils::both(map.steps_to_zzz(), map.steps_to_all_z())
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Map<'a> {
    order: Vec<Direction>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    fn steps_to_zzz(&self) -> usize {
        let mut current = "AAA";
        for (count, direction) in self.order.iter().cycle().enumerate() {
            if current == "ZZZ" {
                return count;
            }
            let (left, right) = self.map.get(current).unwrap();
            current = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
        }
        unreachable!();
    }

    #[allow(dead_code)]
    fn steps_to_all_z(&self) -> usize {
        let mut current_nodes: Vec<&str> = self
            .map
            .keys()
            .filter(|k| k.ends_with('A'))
            .cloned()
            .collect();
        let mut steps_to_end: Vec<usize> = vec![0; current_nodes.len()];
        for (count, direction) in self.order.iter().cycle().enumerate() {
            let mut nodes: Vec<&str> = Vec::with_capacity(current_nodes.len());
            // eprintln!("{:?}\t{:?}\t{:?}", count, current_nodes, steps_to_end);
            for (idx, n) in current_nodes.iter().enumerate() {
                let next = if n.ends_with('Z') {
                    if steps_to_end[idx] == 0 {
                        steps_to_end[idx] = count;
                    }
                    n
                } else {
                    let (left, right) = self.map.get(n).unwrap();
                    match direction {
                        Direction::Left => left,
                        Direction::Right => right,
                    }
                };
                nodes.push(next);
            }
            // Found a count for each node, so break.
            if steps_to_end.iter().all(|&n| n != 0) {
                break;
            }
            // Safety valve.
            if count > 1_000_000 {
                break;
            }
            current_nodes = nodes;
        }
        lcm(&steps_to_end)
    }
}

fn parse_input(test_input: &str) -> Map<'_> {
    let mut lines = test_input.lines();

    let order: Vec<Direction> = lines
        .next()
        .expect("Empty input.")
        .chars()
        .map(Direction::from)
        .collect();
    lines.next(); // Skip blank line.

    let map = lines
        .map(|line| {
            let (front, back) = line.split_once(" = ").unwrap();
            let (left, right) = back
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.split_once(", "))
                .unwrap();
            (front, (left, right))
        })
        .collect();

    Map { map, order }
}

#[cfg(test)]
mod test {
    use super::parse_input;
    use crate::utils::lcm;

    const TEST_INPUT_1: &str = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_3: &str = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)";

    #[test]
    fn day8_parse_test_input() {
        let parsed = parse_input(TEST_INPUT_1);
        assert_eq!(parsed.map["AAA"], ("BBB", "CCC"));
    }

    #[test]
    fn day8_part_one_test_reach_zzz_lr() {
        let map = parse_input(TEST_INPUT_1);
        let steps = map.steps_to_zzz();
        assert_eq!(steps, 2);
    }

    #[test]
    fn day8_part_one_test_reach_zzz_llr() {
        let map = parse_input(TEST_INPUT_2);
        let steps = map.steps_to_zzz();
        assert_eq!(steps, 6);
    }

    #[test]
    fn day8_part_one_real_input() {
        let map = parse_input(super::INPUT);
        let steps = map.steps_to_zzz();
        assert_eq!(steps, 22199);
    }

    #[test]
    fn day8_test_part_two_parse() {
        let parsed = parse_input(TEST_INPUT_3);
        assert_eq!(parsed.map["22A"], ("22B", "XXX"));
        assert_eq!(parsed.map["XXX"], ("XXX", "XXX"));
    }

    #[test]
    fn day8_test_part_two() {
        let map = parse_input(TEST_INPUT_3);
        let expected = 6;
        let result = map.steps_to_all_z();
        assert_eq!(result, expected);
    }

    #[test]
    fn day8_test_lcm() {
        assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
    }

    #[test]
    fn day8_real_part_two() {
        let map = parse_input(super::INPUT);
        let expected: usize = 13334102464297;
        let result = map.steps_to_all_z();
        assert_eq!(expected, result);
    }
}
