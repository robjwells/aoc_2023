use counter::Counter;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use crate::utils;

const INPUT: &str = include_str!("input/2023_07.txt");

pub fn run() -> String {
    let hands = parse_input(INPUT);
    utils::both(part_one(hands.clone()), part_two(hands))
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(cards, bid)| {
            let cards = cards
                .chars()
                .flat_map(Card::try_from)
                .collect::<Vec<_>>()
                .try_into()
                .expect("Couldn't unpack parsed line into [Card; 5]");
            let bid = bid.parse().unwrap();
            Hand::new(cards, bid)
        })
        .collect()
}

fn part_one(mut hands: Vec<Hand>) -> u32 {
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
        .sum()
}

fn part_two(hands: Vec<Hand>) -> u32 {
    let hands = hands.into_iter().map(Hand::jack_to_joker).collect();
    part_one(hands)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    effective_cards: [Card; 5],
    underlying_cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        Hand {
            effective_cards: cards,
            underlying_cards: cards,
            bid,
        }
    }

    fn find_replacement_card(cards: &[Card]) -> Card {
        let mut counter: Counter<_> = cards.iter().collect();
        let joker_count = counter.remove(&Card::Joker).unwrap_or_default();
        if joker_count == 5 {
            // All jokers, so just return the highest possible card.
            Card::King
        } else {
            // Match jokers to the most-common non-joker card.
            let &(&most_common, _) = counter.most_common().first().unwrap();
            most_common
        }
    }

    fn jack_to_joker(self) -> Self {
        let underlying = self.underlying_cards.map(Card::jack_to_joker);
        let replacement = Hand::find_replacement_card(&underlying);
        let effective = underlying.map(|c| if c.is_joker() { replacement } else { c });
        Hand {
            effective_cards: effective,
            underlying_cards: underlying,
            bid: self.bid,
        }
    }

    fn hand_type(&self) -> HandType {
        let counter: Counter<&Card> = self.effective_cards.iter().collect();
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
        let type_cmp = self.hand_type().cmp(&other.hand_type());
        if type_cmp.is_eq() {
            // Use the underlying card values to break ties.
            // For part one, the effective cards == the underlying cards.
            // For part two, the effective cards may be stronger than
            // the underlying cards (due to the presence of jokers).
            self.underlying_cards.cmp(&other.underlying_cards)
        } else {
            type_cmp
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Joker,
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn jack_to_joker(self) -> Self {
        match self {
            Card::Jack => Card::Joker,
            _ => self,
        }
    }

    fn is_joker(&self) -> bool {
        self == &Card::Joker
    }
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = match *self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(n) => n,
            Card::Joker => 1,
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

#[cfg(test)]
mod test {
    use crate::day_07::{parse_input, part_one, part_two, Hand, HandType};

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
            Hand::new([Number(3), Number(2), Number(10), Number(3), King], 765),
            Hand::new([Number(10), Number(5), Number(5), Jack, Number(5)], 684),
            Hand::new([King, King, Number(6), Number(7), Number(7)], 28),
            Hand::new([King, Number(10), Jack, Jack, Number(10)], 220),
            Hand::new([Queen, Queen, Queen, Jack, Ace], 483),
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

    #[test]
    fn day7_part1_winnings() {
        let hands = parse_input(TEST_INPUT);
        assert_eq!(part_one(hands), 6440);
    }

    #[test]
    fn day7_jack_to_joker() {
        use super::Card::*;

        let q = Queen.jack_to_joker();
        assert_eq!(q, Queen);

        let j = Jack.jack_to_joker();
        assert_eq!(j, Joker);
    }

    #[test]
    fn day7_test_input_parse() {
        let hands: Vec<Hand> = parse_input(TEST_INPUT)
            .into_iter()
            .map(|h| h.jack_to_joker())
            .collect();
        assert_eq!(hands[0].hand_type(), HandType::OnePair, "32T3K");
        assert_eq!(hands[1].hand_type(), HandType::FourOfAKind, "T55J5");
        assert_eq!(hands[2].hand_type(), HandType::TwoPair, "KK677");
        assert_eq!(hands[3].hand_type(), HandType::FourOfAKind, "KTJJT");
        assert_eq!(hands[4].hand_type(), HandType::FourOfAKind, "QQQJA");
    }

    #[test]
    fn day7_part2_winnings() {
        let hands = parse_input(TEST_INPUT);
        assert_eq!(part_two(hands), 5905);
    }

    #[test]
    fn day7_real_part1() {
        let hands = parse_input(super::INPUT);
        assert_eq!(part_one(hands), 248105065);
    }
}
