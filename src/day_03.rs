use crate::utils;
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::RangeInclusive,
};

use itertools::Itertools;

const INPUT: &str = include_str!("input/2023_03.txt");

pub fn run() -> String {
    let grid = parse::parse_input(INPUT);
    utils::both(grid.sum_of_part_numbers(), grid.total_gear_ratio())
}

struct Grid {
    map: HashMap<(usize, usize), Element>,
}

impl Grid {
    fn at(&self, pos: (usize, usize)) -> Option<&Element> {
        self.map.get(&pos)
    }

    fn sum_of_part_numbers(&self) -> u32 {
        self.map
            .values()
            .filter(|el| el.kind.is_symbol())
            .flat_map(|el| el.position.potential_adjacent_points())
            .filter_map(|pos| self.at(pos))
            .unique()
            .filter_map(|el| el.kind.as_number())
            .sum()
    }

    fn total_gear_ratio(&self) -> u32 {
        self.map
            .values()
            .filter(|el| matches!(el.kind, ElementKind::Symbol('*')))
            .filter_map(|el| {
                let els = el
                    .position
                    .potential_adjacent_points()
                    .filter_map(|pos| self.at(pos).filter(|el| el.kind.is_number()))
                    .collect::<HashSet<_>>();
                (els.len() == 2).then_some(els.into_iter())
            })
            .map(|els| els.filter_map(|el| el.kind.as_number()).product::<u32>())
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column_start: usize,
    column_end: usize,
    offset: usize,
}

impl Position {
    fn columns(&self) -> RangeInclusive<usize> {
        self.column_start..=self.column_end
    }

    fn all_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.columns().map(|column| (self.row, column))
    }

    fn potential_adjacent_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.all_points().flat_map(|(row, col)| {
            // Doesn't underflow in practice, faster than saturting sub.
            [
                (row + 1, col - 1),
                (row + 1, col),
                (row + 1, col + 1),
                (row, col - 1),
                (row, col + 1),
                (row - 1, col - 1),
                (row - 1, col),
                (row - 1, col + 1),
            ]
            .into_iter()
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Element {
    position: Position,
    kind: ElementKind,
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.position.offset == other.position.offset
    }
}

impl Eq for Element {}

impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.position.offset)
    }
}

#[derive(Debug, Clone, Copy)]
enum ElementKind {
    Symbol(char),
    Number(u32),
}

impl ElementKind {
    /// Returns `true` if the element kind is [`Number`].
    ///
    /// [`Number`]: ElementKind::Number
    #[must_use]
    fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    /// Returns `true` if the element kind is [`Symbol`].
    ///
    /// [`Symbol`]: ElementKind::Symbol
    #[must_use]
    fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(..))
    }

    fn as_number(&self) -> Option<&u32> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

mod parse {
    use std::collections::HashMap;

    use super::{Element, ElementKind, Grid, Position};
    use nom::{
        branch::alt,
        bytes::complete::is_a,
        character::complete::{anychar, digit1, line_ending},
        combinator::value,
        multi::many1,
        IResult,
    };
    use nom_locate::{position, LocatedSpan};

    type Span<'a> = LocatedSpan<&'a str>;

    fn element_with_symbol(value: char, position: Span) -> Element {
        let col = position.get_column() - 1;
        let row = (position.location_line() - 1) as usize;
        Element {
            position: Position {
                row,
                column_start: col,
                column_end: col,
                offset: position.location_offset(),
            },
            kind: ElementKind::Symbol(value),
        }
    }

    fn element_with_number(value: u32, length: usize, position: Span) -> Element {
        let col = position.get_column() - 1;
        let row = (position.location_line() - 1) as usize;
        Element {
            position: Position {
                row,
                column_start: col,
                column_end: col + length - 1,
                offset: position.location_offset(),
            },
            kind: ElementKind::Number(value),
        }
    }

    pub fn parse_input(input: &str) -> Grid {
        let (_remaining, results) = parse(input.into()).expect("Parsing failed.");
        let mut map = HashMap::new();
        for el in results.into_iter().flatten() {
            for point in el.position.all_points() {
                map.insert(point, el);
            }
        }
        Grid { map }
    }

    fn parse(input: Span) -> IResult<Span, Vec<Option<Element>>> {
        many1(alt((
            number,
            value(None, is_a(".")),
            value(None, line_ending),
            symbol,
        )))(input)
    }

    fn symbol(input: Span) -> IResult<Span, Option<Element>> {
        let (remaining, position) = position(input)?;
        let (remaining, symbol) = anychar(remaining)?;
        Ok((remaining, Some(element_with_symbol(symbol, position))))
    }

    fn number(input: Span) -> IResult<Span, Option<Element>> {
        let (remaining, position) = position(input)?;
        let (remaining, number) = digit1(remaining)?;
        let length = number.len();
        let value = number.parse().unwrap();
        let res = element_with_number(value, length, position);
        Ok((remaining, Some(res)))
    }
}

#[cfg(test)]
mod test {
    use super::parse::parse_input;
    use super::{Element, ElementKind, Grid, Position, INPUT as REAL_INPUT};

    const TEST_INPUT: &str = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

    #[test]
    fn day3_parse_test_elements() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(
            grid.at((0, 0)),
            Some(&Element {
                position: Position {
                    row: 0,
                    column_start: 0,
                    column_end: 2,
                    offset: 0,
                },
                kind: ElementKind::Number(467)
            })
        );
        assert_eq!(
            grid.at((1, 3)),
            Some(&Element {
                position: Position {
                    row: 1,
                    column_start: 3,
                    column_end: 3,
                    offset: 14,
                },
                kind: ElementKind::Symbol('*')
            })
        );
    }

    #[test]
    fn day3_sum_of_test_part_numbers() {
        let grid: Grid = parse_input(TEST_INPUT);
        let sum: u32 = grid.sum_of_part_numbers();
        assert_eq!(sum, 4361);
    }

    #[test]
    fn day3_sum_of_real_part_numbers() {
        let grid: Grid = parse_input(REAL_INPUT);
        let sum: u32 = grid.sum_of_part_numbers();
        assert_eq!(sum, 535_235);
    }

    #[test]
    fn day3_test_total_gear_ratio() {
        let grid: Grid = parse_input(TEST_INPUT);
        let gear_ratio: u32 = grid.total_gear_ratio();
        assert_eq!(gear_ratio, 467_835);
    }

    #[test]
    fn day3_real_total_gear_ratio() {
        let grid: Grid = parse_input(REAL_INPUT);
        let gear_ratio: u32 = grid.total_gear_ratio();
        assert_eq!(gear_ratio, 79_844_424);
    }
}
