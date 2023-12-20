#![allow(unused, dead_code)]
use counter::Counter;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use crate::utils;

const INPUT: &str = include_str!("input/2023-07.txt");

pub fn run() -> String {
    utils::first(part_one(INPUT))
}

fn part_one(input: &str) -> u32 {
    let mut hands = parse_input(input);
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let counter: Counter<&Card> = self.cards.iter().collect();
        let in_order = counter.most_common();
        let (_, count_most_common) = in_order.first().unwrap();
        match count_most_common {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if in_order.len() == 2 {
                    // Three of a kind and a pair
                    HandType::FullHouse
                } else {
                    // Three of a kind and two single cards
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if in_order.len() == 3 {
                    // Two pairs and one single card
                    HandType::TwoPair
                } else {
                    // One pair and three single cards
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!("Minimum count is 1, maximum count is 5"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type() != other.hand_type() {
            self.hand_type().cmp(&other.hand_type())
        } else {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(self_card, other_card)| self_card.cmp(other_card))
                .find(|ordering| ordering.is_ne())
                .unwrap_or(Ordering::Equal)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(n) => *n,
        };
        state.write_u32(n);
    }
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Card::*;
        Ok(match value {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Number(10),
            n @ '2'..='9' => Number(n.to_digit(10).unwrap()),
            _ => return Err("Unknown card character."),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(cards, bid)| {
            let cards: [Card; 5] = cards
                .chars()
                .flat_map(Card::try_from)
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap();
            let bid = bid.parse().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day_07::HandType;

    use super::{parse_input, Hand};

    const TEST_INPUT: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483";

    #[test]
    fn day7_test_parse() {
        use super::Card::*;
        let hands = parse_input(TEST_INPUT);
        let expected_hands = vec![
            Hand {
                cards: [Number(3), Number(2), Number(10), Number(3), King],
                bid: 765,
            },
            Hand {
                cards: [Number(10), Number(5), Number(5), Jack, Number(5)],
                bid: 684,
            },
            Hand {
                cards: [King, King, Number(6), Number(7), Number(7)],
                bid: 28,
            },
            Hand {
                cards: [King, Number(10), Jack, Jack, Number(10)],
                bid: 220,
            },
            Hand {
                cards: [Queen, Queen, Queen, Jack, Ace],
                bid: 483,
            },
        ];

        for (got, expected) in hands.into_iter().zip(expected_hands.into_iter()) {
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn day7_test_hand_type() {
        let hands = parse_input(TEST_INPUT);

        assert_eq!(hands[0].hand_type(), HandType::OnePair);
        assert_eq!(hands[1].hand_type(), HandType::ThreeOfAKind);
        assert_eq!(hands[2].hand_type(), HandType::TwoPair);
        assert_eq!(hands[3].hand_type(), HandType::TwoPair);
        assert_eq!(hands[4].hand_type(), HandType::ThreeOfAKind);

        let hands = parse_input("23232 1\nA9AAA 2");
        assert_eq!(hands[0].hand_type(), HandType::FullHouse);
        assert_eq!(hands[1].hand_type(), HandType::FourOfAKind);
    }
}
