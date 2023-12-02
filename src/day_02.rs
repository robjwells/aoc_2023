use std::str::FromStr;

const PUZZLE_INPUT: &str = include_str!("input/2023_02.txt");

pub fn run() -> String {
    let games = parse_input(PUZZLE_INPUT);
    aoc_2023::both(part_one(&games), part_two(&games))
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().flat_map(Game::from_str).collect()
}

fn part_one(games: &[Game]) -> u16 {
    games
        .iter()
        .map(Game::summarise)
        .filter(GameSummary::meets_minimum)
        .map(|g| g.number as u16)
        .sum()
}

fn part_two(games: &[Game]) -> u16 {
    games.iter().map(|g| g.summarise().power()).sum()
}

#[derive(Debug)]
struct Game {
    number: u8,
    revealed: Vec<Vec<Cube>>,
}

trait SummariseCubes {
    fn summarise(&self) -> CubeSummary;
}

impl SummariseCubes for Vec<Cube> {
    fn summarise(&self) -> CubeSummary {
        self.iter().fold(CubeSummary::new(), CubeSummary::update)
    }
}

impl Game {
    fn summarise(&self) -> GameSummary {
        let summary = self
            .revealed
            .iter()
            .map(|v| v.summarise())
            .fold(CubeSummary::new(), CubeSummary::combine);
        GameSummary {
            number: self.number,
            summary,
        }
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((prefix, suffix)) = s.split_once(": ") else {
            return Err("Missing colon");
        };
        let Some(("Game", number)) = prefix.split_once(' ') else {
            return Err("Failed to destructure Game prefix");
        };
        let Ok(number) = number.parse::<u8>() else {
            return Err("Failed to parse number portion of prefix");
        };

        let revealed: Vec<Vec<_>> = suffix
            .split(';')
            .map(|cubes| {
                cubes
                    .split(',')
                    .map(|s| Cube::from_str(s).unwrap())
                    .collect()
            })
            .collect();

        Ok(Self { number, revealed })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cube {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl FromStr for Cube {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((count, colour)) = s.trim().split_once(' ') else {
            return Err("Failed to split on space");
        };

        let Ok(count) = count.parse::<u8>() else {
            return Err("Failed to parse expected count");
        };

        match colour {
            "red" => Ok(Cube::Red(count)),
            "green" => Ok(Cube::Green(count)),
            "blue" => Ok(Cube::Blue(count)),
            _ => Err("Unexpected colour"),
        }
    }
}

#[derive(Debug)]
struct CubeSummary {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeSummary {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn update(self, cube: &Cube) -> CubeSummary {
        let Self {
            mut red,
            mut green,
            mut blue,
        } = self;
        match cube {
            Cube::Red(n) => red = *n,
            Cube::Green(n) => green = *n,
            Cube::Blue(n) => blue = *n,
        }
        Self { red, green, blue }
    }

    fn combine(self, other: CubeSummary) -> CubeSummary {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u16 {
        self.red as u16 * self.green as u16 * self.blue as u16
    }
}

struct GameSummary {
    number: u8,
    summary: CubeSummary,
}

impl GameSummary {
    fn meets_minimum(&self) -> bool {
        self.summary.red <= 12 && self.summary.green <= 13 && self.summary.blue <= 14
    }

    fn power(&self) -> u16 {
        self.summary.power()
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use crate::day_02::{parse_input, part_one, part_two, PUZZLE_INPUT};

    use super::{Cube, Game, GameSummary};
    use std::str::FromStr;

    #[allow(dead_code)]
    const TEST_INPUT: &str = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn day2_parse_single_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_str(input).unwrap();
        assert_eq!(game.number, 1);
        assert_eq!(game.revealed.len(), 3);
        assert_eq!(
            game.revealed[1].as_slice(),
            &[Cube::Red(1), Cube::Green(2), Cube::Blue(6)]
        );
    }

    #[test]
    fn day2_parse_all_games() {
        let numbers: Vec<u8> = TEST_INPUT
            .lines()
            .flat_map(Game::from_str)
            .map(|g| g.number)
            .collect();
        assert_eq!(numbers.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn day2_parse_real_input() {
        let games: Vec<Game> = super::PUZZLE_INPUT
            .lines()
            .flat_map(Game::from_str)
            .collect();
        assert_eq!(games.len(), 100);
    }

    #[test]
    fn day2_test_part_one() {
        let games = parse_input(TEST_INPUT);
        let sum_qualified = part_one(&games);
        assert_eq!(sum_qualified, 8);
    }

    #[test]
    fn day2_real_part_one() {
        let games = parse_input(PUZZLE_INPUT);
        let sum_qualified = part_one(&games);
        assert_eq!(sum_qualified, 2476);
    }

    #[test]
    fn day_2_power() {
        let games = parse_input(TEST_INPUT);
        let game = games.first().unwrap();
        let power = game.summarise().power();
        assert_eq!(power, 48);

        let total_power = part_two(&games);
        assert_eq!(total_power, 2286);
    }
}
