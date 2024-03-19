#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    iter,
    str::FromStr,
};

const INPUT: &str = include_str!("input/2023_10.txt");

use crate::utils;

const PLAIN_LOOP: &str = "\
    .....\n\
    .S-7.\n\
    .|.|.\n\
    .L-J.\n\
    .....";

pub fn run() -> String {
    let mut m: Map = INPUT.parse().unwrap();
    m.fill_distances();
    utils::first(m.max_distance())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Segment {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl Segment {
    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    fn connects_south(&self) -> bool {
        matches!(self, Self::Vertical | Self::SouthToWest | Self::SouthToEast)
    }

    fn connects_north(&self) -> bool {
        matches!(self, Self::Vertical | Self::NorthToWest | Self::NorthToEast)
    }

    fn connects_west(&self) -> bool {
        matches!(
            self,
            Self::Horizontal | Self::NorthToWest | Self::SouthToWest
        )
    }

    fn connects_east(&self) -> bool {
        matches!(
            self,
            Self::Horizontal | Self::NorthToEast | Self::SouthToEast
        )
    }

    fn neighbours(&self, position: Position) -> [Position; 2] {
        let deltas = match *self {
            Segment::Vertical => [(0, -1), (0, 1)],
            Segment::Horizontal => [(-1, 0), (1, 0)],
            Segment::NorthToEast => [(0, -1), (1, 0)],
            Segment::NorthToWest => [(0, -1), (-1, 0)],
            Segment::SouthToWest => [(0, 1), (-1, 0)],
            Segment::SouthToEast => [(0, 1), (1, 0)],
            _ => unreachable!(),
        };
        deltas.map(|(xd, yd)| (position.0 + xd, position.1 + yd))
    }
}

fn resolve_neighbours(
    above: Option<Segment>,
    right: Option<Segment>,
    below: Option<Segment>,
    left: Option<Segment>,
) -> Segment {
    use Segment::*;
    match (above, right, below, left) {
        (Some(a), Some(r), _, _) if a.connects_south() && r.connects_west() => NorthToEast,
        (Some(a), _, Some(b), _) if a.connects_south() && b.connects_north() => Vertical,
        (Some(a), _, _, Some(l)) if a.connects_south() && l.connects_east() => NorthToWest,
        (_, Some(r), Some(b), _) if r.connects_west() && b.connects_north() => SouthToEast,
        (_, Some(r), _, Some(l)) if r.connects_west() && l.connects_east() => Horizontal,
        (_, _, Some(b), Some(l)) if b.connects_north() && l.connects_east() => SouthToWest,
        _ => unreachable!(),
    }
}

impl From<char> for Segment {
    fn from(value: char) -> Self {
        use Segment::*;
        match value {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthToEast,
            'J' => NorthToWest,
            '7' => SouthToWest,
            'F' => SouthToEast,
            '.' => Ground,
            'S' => Start,
            _ => unreachable!(),
        }
    }
}

type Position = (isize, isize);

struct Map {
    map: HashMap<Position, Segment>,
    distances: HashMap<Position, usize>,
    start: Position,
}

impl Map {
    fn new(map: HashMap<Position, Segment>, start: Position) -> Self {
        let distances = map.keys().copied().zip(iter::repeat(0)).collect();
        Self {
            map,
            distances,
            start,
        }
    }

    fn at(&self, pos: Position) -> Option<Segment> {
        self.map.get(&pos).copied()
    }

    fn next_neighbour(&self, current: Position) -> Option<Position> {
        self.map[&current]
            .neighbours(current)
            .into_iter()
            .find(|pos| *pos != self.start && self.distances[pos] == 0)
    }

    fn fill_distances(&mut self) {
        let potentials = self.map[&self.start].neighbours(self.start);
        let mut cost = 1;
        let mut current = potentials[0];
        self.distances.insert(current, cost);
        while let Some(pos) = self.next_neighbour(current) {
            cost += 1;
            current = pos;
            self.distances.insert(current, cost);
        }
    }

    fn max_distance(&mut self) -> usize {
        if self.distances.values().all(|&d| d == 0) {
            self.fill_distances();
        }
        self.distances.values().max().unwrap() / 2 + 1
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.map.keys().map(|p| p.0).max().unwrap();
        let mut out = String::new();
        for y in 0..size {
            for x in 0..size {
                let distance = {
                    if (x, y) == self.start {
                        "S".to_owned()
                    } else {
                        let d = self.distances[&(x, y)];
                        if d == 0 {
                            " ".to_owned()
                        } else {
                            d.to_string()
                        }
                    }
                };
                out.push_str(&distance);
            }
            out.push('\n');
        }
        f.write_str(&out)
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s.find('\n').unwrap() as isize;
        let mut map = HashMap::with_capacity(size.pow(2) as usize);
        let mut start: Option<Position> = None;
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let segment: Segment = c.into();
                if segment.is_start() {
                    start.replace((x as isize, y as isize));
                }
                map.insert((x as isize, y as isize), segment);
            }
        }
        let start = start.expect("Start position not found.");

        // Now replace the start position with the actual segment.
        let (x, y) = start;
        let start_segment = resolve_neighbours(
            (y > 0)
                .then_some((x, y - 1))
                .and_then(|o| map.get(&o).copied()),
            (x < size - 1)
                .then_some((x + 1, y))
                .and_then(|o| map.get(&o).copied()),
            (y < size - 1)
                .then_some((x, y + 1))
                .and_then(|o| map.get(&o).copied()),
            (x > 0)
                .then_some((x - 1, y))
                .and_then(|o| map.get(&o).copied()),
        );
        map.insert(start, start_segment);
        Ok(Self::new(map, start))
    }
}

#[cfg(test)]
mod test {
    use super::{Map, Segment};

    const PLAIN_LOOP: &str = "\
        .....\n\
        .S-7.\n\
        .|.|.\n\
        .L-J.\n\
        .....";
    const COMPLEX_LOOP: &str = "\
        ..F7.\n\
        .FJ|.\n\
        SJ.L7\n\
        |F--J\n\
        LJ...";

    #[test]
    fn day10_test_parse() {
        let mut map: Map = PLAIN_LOOP.parse().unwrap();
        let expected = Segment::Vertical;
        let result = map.at((1, 2)).unwrap();
        assert_eq!(expected, result);
        assert_eq!(map.start, (1, 1));
        map.fill_distances();
    }

    #[test]
    fn day10_test_simple_distance() {
        eprintln!("Simple");
        let mut map: Map = PLAIN_LOOP.parse().unwrap();
        let distance = map.max_distance();
        eprintln!("{}", map);
        assert_eq!(distance, 4);
    }

    #[test]
    fn day10_test_complex_distance() {
        eprintln!("Complex");
        let mut map: Map = COMPLEX_LOOP.parse().unwrap();
        let distance = map.max_distance();
        assert_eq!(distance, 8);
    }

    #[test]
    fn day10_segment_connections() {
        assert!(Segment::Vertical.connects_north());
        assert!(Segment::Vertical.connects_south());
        assert!(Segment::SouthToEast.connects_south());
        assert!(Segment::SouthToEast.connects_east());
    }
}
