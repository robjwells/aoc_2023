use crate::utils;

const INPUT: &str = include_str!("input/2023_06.txt");

pub fn run() -> String {
    utils::both(part_one(INPUT), part_two(INPUT))
}

fn part_one(input: &str) -> u64 {
    parse_multiple_races(input)
        .into_iter()
        .map(|r| r.ways_to_win())
        .product()
}

fn part_two(input: &str) -> u64 {
    parse_single_race(input).ways_to_win()
}

#[derive(Debug, PartialEq)]
struct Race {
    time_limit: f64,
    distance_record: f64,
}

impl Race {
    #[allow(dead_code)]
    fn from_pair((time_limit, distance_record): (f64, f64)) -> Self {
        Self {
            time_limit,
            distance_record,
        }
    }

    /// From mnvr on GitHub.
    ///
    /// Quadratic equation:
    ///  T = time limit
    ///  h = hold time (speed)
    ///  D = distance record
    ///      -h^2 + Th - D
    ///
    ///  Quadratic formula, to solve for h:
    ///      ± (√ T^2 - 4D) + T
    #[allow(unused, dead_code)]
    fn calculate_hold_time_bounds(&self) -> (u64, u64) {
        let t = self.time_limit;
        let d = self.distance_record;

        let lower = (-(t * t - 4.0 * d).sqrt() + t) / 2.0;
        let upper = ((t * t - 4.0 * d).sqrt() + t) / 2.0;

        // Handle the case where lower & upper are already integers,
        // by reducing the range inwards by 1 on each side.
        let lower = {
            let ceil = lower.ceil();
            (if ceil != lower { ceil } else { lower + 1.0 }) as u64
        };
        let upper = {
            let floor = upper.floor();
            (if floor != upper { floor } else { upper - 1.0 }) as u64
        };

        (lower, upper)
    }

    fn ways_to_win(&self) -> u64 {
        let (lower, upper) = self.calculate_hold_time_bounds();
        upper - lower + 1
    }
}

fn parse_multiple_races(input: &str) -> Vec<Race> {
    let [times, distances]: [Vec<f64>; 2] = input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(_prefix, numbers)| numbers.split_whitespace())
        .map(|numbers| numbers.map(|s| s.parse::<f64>().unwrap()).collect())
        .collect::<Vec<Vec<f64>>>()
        .try_into()
        .expect("Couldn't parse input as two lines of numbers.");

    times
        .into_iter()
        .zip(distances)
        .map(Race::from_pair)
        .collect()
}

fn parse_single_race(input: &str) -> Race {
    let [time_limit, distance_record]: [f64; 2] = input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(_prefix, line)| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|n| n as u64))
                .fold(0, |acc, next| acc * 10 + next) as f64
        })
        .collect::<Vec<f64>>()
        .try_into()
        .expect("Couldn't parse input as two numbers.");

    Race {
        time_limit,
        distance_record,
    }
}

#[cfg(test)]
mod test {
    use super::{parse_multiple_races, part_one, part_two, Race, INPUT};
    const TEST_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn day6_test_parse() {
        let races = parse_multiple_races(TEST_INPUT);
        assert_eq!(
            races,
            vec![
                Race::from_pair((7.0, 9.0)),
                Race::from_pair((15.0, 40.0)),
                Race::from_pair((30.0, 200.0))
            ]
        );
    }

    #[test]
    fn day6_test_calculate() {
        let (min_hold, max_hold) = Race {
            time_limit: 7.0,
            distance_record: 9.0,
        }
        .calculate_hold_time_bounds();
        assert_eq!(min_hold, 2);
        assert_eq!(max_hold, 5);
    }

    #[test]
    fn day6_test_ways_to_win() {
        let races = parse_multiple_races(TEST_INPUT);
        let answers: Vec<u64> = races.into_iter().map(|r| r.ways_to_win()).collect();
        assert_eq!(&answers, &[4, 8, 9]);
    }

    #[test]
    fn day6_test_part_one() {
        let answer = part_one(TEST_INPUT);
        assert_eq!(answer, 288);
    }

    #[test]
    fn day6_test_part_two() {
        let answer = part_two(TEST_INPUT);
        assert_eq!(answer, 71503);
    }

    #[test]
    fn day6_real_part_one() {
        assert_eq!(part_one(INPUT), 2449062);
    }

    #[test]
    fn day6_real_part_two() {
        assert_eq!(part_two(INPUT), 33149631);
    }
}
