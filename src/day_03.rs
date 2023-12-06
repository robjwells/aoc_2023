use std::{collections::HashSet, ops::Index, str::FromStr};

const PUZZLE_INPUT: &str = include_str!("input/2023_03.txt");

pub fn run() -> String {
    let grid: Grid = PUZZLE_INPUT.parse().expect("Failed to parse real input");
    let part_one: u32 = grid.sum_of_part_numbers();
    let part_two: u32 = grid.total_gear_ratio();
    aoc_2023::both(part_one, part_two)
}

#[derive(Debug)]
struct Grid {
    elements: Vec<Element>,
    indices: Vec<Vec<usize>>,
}

impl Index<Point> for Grid {
    type Output = Element;

    fn index(&self, location: Point) -> &Self::Output {
        let Point {
            row: original_row,
            column: original_column,
        } = location;
        let idx = self.indices[original_row][original_column];
        &self.elements[idx]
    }
}

impl Grid {
    fn rows_len(&self) -> usize {
        self.indices.len()
    }

    fn cols_len(&self) -> usize {
        self.indices[0].len()
    }

    fn symbols(&self) -> impl Iterator<Item = &Element> {
        self.elements.iter().filter(|e| e.is_symbol())
    }

    fn symbols_with_part_numbers(&self) -> Vec<(&Element, Vec<&Element>)> {
        let mut pairs = Vec::new();
        for symbol in self.symbols() {
            let unique_adjacents: HashSet<&Element> = symbol
                .positions()
                .into_iter()
                .flat_map(|p| p.adjacents(self.rows_len(), self.cols_len()))
                .map(|p| &self[p])
                .filter(|e| e.is_number())
                .collect();
            pairs.push((symbol, unique_adjacents.into_iter().collect()));
        }
        pairs
    }

    fn sum_of_part_numbers(&self) -> u32 {
        let unique_parts: HashSet<_> = self
            .symbols_with_part_numbers()
            .into_iter()
            .flat_map(|(_sym, els)| els)
            .collect();

        unique_parts
            .into_iter()
            .filter_map(|el| {
                if let Element::Number { value, .. } = el {
                    Some(value)
                } else {
                    None
                }
            })
            .sum()
    }

    fn total_gear_ratio(&self) -> u32 {
        self.symbols_with_part_numbers()
            .into_iter()
            .filter(|(sym, els)| sym.is_gear() && els.len() == 2)
            .map(|(_sym, els)| {
                els.into_iter().filter_map(|el| {
                    if let Element::Number { value, .. } = el {
                        Some(value)
                    } else {
                        None
                    }
                })
            })
            .map(|ns| ns.product::<u32>())
            .sum()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Element {
    Number { value: u32, positions: Vec<Point> },
    Symbol { value: char, position: Point },
    Blank { position: Point },
}

impl Element {
    /// Returns `true` if the element is [`Number`].
    ///
    /// [`Number`]: Element::Number
    #[must_use]
    fn is_number(&self) -> bool {
        matches!(self, Self::Number { .. })
    }

    fn positions(&self) -> Vec<Point> {
        match self {
            Element::Number { positions, .. } => positions.clone(),
            Element::Symbol { position, .. } => [*position].into(),
            Element::Blank { position } => [*position].into(),
        }
    }

    /// Returns `true` if the element is [`Symbol`].
    ///
    /// [`Symbol`]: Element::Symbol
    #[must_use]
    fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol { .. })
    }

    fn is_gear(&self) -> bool {
        matches!(self, Element::Symbol { value: '*', .. })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn adjacents(&self, rows: usize, columns: usize) -> Vec<Self> {
        HashSet::from([
            (self.row.saturating_sub(1), self.column.saturating_sub(1)),
            (self.row.saturating_sub(1), self.column),
            (self.row.saturating_sub(1), self.column + 1),
            (self.row, self.column.saturating_sub(1)),
            (self.row, self.column + 1),
            (self.row + 1, self.column.saturating_sub(1)),
            (self.row + 1, self.column),
            (self.row + 1, self.column + 1),
        ])
        .into_iter()
        .map(|(row, col)| Point::new(row, col))
        .filter(|p| p.row < rows && p.column < columns)
        .collect()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.row == other.row {
            self.column.cmp(&other.column)
        } else {
            self.row.cmp(&other.row)
        }
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    #[allow(unused_variables)]
    fn from_str(s: &str) -> Result<Grid, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let width = lines[0].len();
        let height = lines.len();

        let mut out: Vec<Vec<Element>> = (0..height).map(|_| Vec::with_capacity(width)).collect();

        for (row, line) in lines.iter().enumerate() {
            for (column, next) in line.chars().enumerate() {
                let res = match next {
                    '.' => Element::Blank {
                        position: Point::new(row, column),
                    },
                    n if next.is_ascii_digit() => Element::Number {
                        value: n.to_digit(10).unwrap(),
                        positions: vec![Point::new(row, column)],
                    },
                    s if next.is_ascii_punctuation() => Element::Symbol {
                        value: s,
                        position: Point::new(row, column),
                    },
                    other => panic!("Unexpected character {:?}", other),
                };
                debug_assert!(out[row].len() == column);
                out[row].push(res);
            }
        }

        let mut elements = Vec::with_capacity(height);
        for row in out.into_iter() {
            let mut column = row.into_iter().peekable();
            while let Some(next) = column.next() {
                if let Element::Number { value, positions } = next {
                    let mut n = value;
                    let mut pos = positions;

                    while column.peek().is_some_and(Element::is_number) {
                        let Some(Element::Number { value, positions }) = column.next() else {
                            unreachable!()
                        };
                        n = n * 10 + value;
                        pos.extend(positions);
                    }

                    pos.sort_unstable();
                    elements.push(Element::Number {
                        value: n,
                        positions: pos,
                    });
                } else {
                    elements.push(next);
                }
            }
        }

        let mut indices: Vec<Vec<usize>> = {
            let mut v = Vec::with_capacity(height);
            for _ in 0..height {
                v.push(Vec::with_capacity(width));
            }
            v
        };
        for (element_idx, element) in elements.iter().enumerate() {
            for Point { row, column } in element.positions() {
                debug_assert!(indices[row].len() == column);
                indices[row].push(element_idx);
            }
        }
        Ok(Self { elements, indices })
    }
}

#[cfg(test)]
mod test {
    use crate::day_03::PUZZLE_INPUT;

    use super::{Element, Grid, Point};

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
    fn day3_parse_test_table() {
        let grid: Grid = TEST_INPUT.parse().unwrap();
        let Element::Symbol { value, position } = grid[Point::new(1, 3)] else {
            panic!("Symbol not found at expected location.")
        };
        assert_eq!(value, '*');
        assert_eq!(position, Point::new(1, 3));

        let Element::Number { value, positions } = &grid[Point::new(4, 1)] else {
            panic!("Number not found at expected location.")
        };
        assert_eq!(*value, 617);
        assert_eq!(
            positions,
            &[Point::new(4, 0), Point::new(4, 1), Point::new(4, 2)]
        );
    }

    #[test]
    fn day3_sum_of_test_part_numbers() {
        let grid: Grid = TEST_INPUT.parse().unwrap();
        let sum: u32 = grid.sum_of_part_numbers();
        assert_eq!(sum, 4361);
    }

    #[test]
    fn day3_sum_of_real_part_numbers() {
        let grid: Grid = PUZZLE_INPUT.parse().unwrap();
        let sum: u32 = grid.sum_of_part_numbers();
        assert_eq!(sum, 535_235);
    }

    #[test]
    fn day3_test_total_gear_ratio() {
        let grid: Grid = TEST_INPUT.parse().unwrap();
        let gear_ratio: u32 = grid.total_gear_ratio();
        assert_eq!(gear_ratio, 467_835);
    }
}
