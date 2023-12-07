use crate::utils;
use std::{collections::HashSet, str::FromStr, vec};

const INPUT: &str = include_str!("input/2023_04.txt");

pub fn run() -> String {
    let cards = parse_input(INPUT);
    utils::both(part_one(&cards), part_two(&cards))
}

fn part_one(cards: &[Card]) -> usize {
    cards.iter().map(Card::points).sum()
}

fn part_two(cards: &[Card]) -> usize {
    let mut counts = vec![1_usize; cards.len()];
    cards
        .iter()
        .map(Card::number_of_winners)
        .enumerate()
        .for_each(|(current_idx, winners)| {
            for new_idx in current_idx + 1..=current_idx + winners {
                // We get 1 of each new card for each of the current card.
                counts[new_idx] += counts[current_idx];
            }
        });
    counts.iter().sum()
}

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: usize,
    winners: HashSet<u8>,
    candidates: HashSet<u8>,
}

impl Card {
    fn number_of_winners(&self) -> usize {
        self.candidates.intersection(&self.winners).count()
    }

    fn points(&self) -> usize {
        match self.number_of_winners() as u32 {
            0 => 0,
            n => 2_usize.pow(n - 1),
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

        let id = id
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
    use crate::day_04::{part_one, part_two};

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
        let points = part_one(&cards);
        assert_eq!(points, 13);
    }

    #[test]
    fn day4_test_total_cards() {
        let cards = parse_input(TEST_INPUT);
        let answer = part_two(&cards);
        assert_eq!(answer, 30);
    }
}
