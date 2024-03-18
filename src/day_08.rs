use std::collections::HashMap;

use crate::utils;

const INPUT: &str = include_str!("input/2023-08.txt");

pub fn run() -> String {
    utils::first(part_one(INPUT))
}

fn part_one(input: &str) -> usize {
    parse_input(input).steps_to_zzz()
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
    use crate::day_08::part_one;

    use super::parse_input;

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

    #[test]
    fn day8_parse_test_input() {
        let parsed = parse_input(TEST_INPUT_1);
        assert_eq!(parsed.map["AAA"], ("BBB", "CCC"));
    }

    #[test]
    fn day8_part_one_test_reach_zzz_lr() {
        let steps = part_one(TEST_INPUT_1);
        assert_eq!(steps, 2);
    }

    #[test]
    fn day8_part_one_test_reach_zzz_llr() {
        let steps = part_one(TEST_INPUT_2);
        assert_eq!(steps, 6);
    }

    #[test]
    fn day8_part_one_real_input() {
        let steps = part_one(super::INPUT);
        assert_eq!(steps, 22199);
    }
}
