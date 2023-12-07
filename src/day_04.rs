use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("input/2023_04.txt");

pub fn run() -> String {
    let cards = parse_input(INPUT);
    let points: Vec<u64> = cards.iter().map(Card::points).collect();
    aoc_2023::first(points.iter().sum::<u64>())
}

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: u8,
    winners: HashSet<u8>,
    candidates: HashSet<u8>,
}

impl Card {
    fn number_of_winners(&self) -> u32 {
        self.candidates.intersection(&self.winners).count() as u32
    }

    fn points(&self) -> u64 {
        match self.number_of_winners() {
            0 => 0,
            n => 2_u64.pow(n - 1),
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((prefix, numbers)) = s.split_once(':') else {
            Err("No card prefix found.")?
        };

        let Some((_, id)) = prefix.split_once(' ') else {
            Err("Card prefix is incorrect.")?
        };

        let id: u8 = id
            .trim()
            .parse()
            .map_err(|_e| "Failed to parse card id as a number")?;

        let Some((winners, candidates)) = numbers.split_once('|') else {
            Err("Failed to find winning and candidate numbers.")?
        };

        let winners = winners.split_whitespace().flat_map(|w| w.parse()).collect();
        let candidates = candidates
            .split_whitespace()
            .flat_map(|w| w.parse())
            .collect();

        Ok(Card {
            id,
            winners,
            candidates,
        })
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    input.lines().flat_map(|line| line.parse()).collect()
}

#[cfg(test)]
mod test {
    use super::{parse_input, Card};

    const TEST_INPUT: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn day4_parse_test_input() {
        let cards: Vec<Card> = parse_input(TEST_INPUT);
        assert_eq!(cards[0].id, 1);
        assert_eq!(cards[0].winners, [41, 48, 83, 86, 17].into());
        assert_eq!(cards[0].candidates, [83, 86, 6, 31, 17, 9, 48, 53].into());
        assert_eq!(cards[5].id, 6);
        assert_eq!(cards[5].winners, [31, 18, 13, 56, 72].into());
        assert_eq!(cards[5].candidates, [74, 77, 10, 23, 35, 67, 36, 11].into());
    }

    #[test]
    fn day4_test_points() {
        let cards = parse_input(TEST_INPUT);
        let total: u64 = cards.iter().map(|c| c.points()).sum();
        assert_eq!(total, 13);
    }
}
