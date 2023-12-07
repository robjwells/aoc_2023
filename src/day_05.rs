use crate::utils;
use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::value,
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

const INPUT: &str = include_str!("input/2023_05.txt");

pub fn run() -> String {
    let (seeds, maps) = parse_input_single_seeds(INPUT).unwrap().1;
    let p1 = part_one(&seeds, &maps);

    let (seeds, maps) = parse_input_seed_ranges(INPUT).unwrap().1;
    let p2 = chain_ranges(&seeds, &maps).0;

    utils::both(p1, p2)
}

fn part_one(seeds: &[Seed], maps: &[Map]) -> u64 {
    seeds.iter().map(|s| chain(s, maps)).min().unwrap().0
}

#[derive(Debug, PartialEq, Eq)]
enum Seed {
    Single(u64),
    Range(Range<u64>),
}

impl Seed {
    #[allow(dead_code)]
    fn as_single(&self) -> Option<&u64> {
        if let Self::Single(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn as_range(&self) -> Option<&Range<u64>> {
        if let Self::Range(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    #[allow(dead_code)]
    kind: MapKind,
    lines: Vec<MapLine>,
}

impl Map {
    fn translate(&self, src: u64) -> u64 {
        self.lines
            .iter()
            .find(|ml| ml.in_source(src))
            .map(|line| line.translate(src))
            .unwrap_or(src)
    }

    // From u/legobmw99 on Reddit.
    fn translate_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut mapped = Vec::new();
        let mut remaining = vec![range];

        while let Some(candidate) = remaining.pop() {
            let mut did_something = false;
            for m in &self.lines {
                if let (Some(mapped_range), leftover_ranges) =
                    m.translate_subrange(candidate.clone())
                {
                    mapped.push(mapped_range);
                    remaining.extend(leftover_ranges);
                    did_something = true;
                    break;
                }
            }
            // No need to convert the range.
            if !did_something {
                mapped.push(candidate);
            }
        }
        mapped
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Location(u64);

fn chain(seed: &Seed, maps: &[Map]) -> Location {
    let Seed::Single(mut value) = seed else {
        return Location(u64::MAX);
    };
    for map in maps {
        value = map.translate(value);
    }
    Location(value)
}

// From u/legobmw99 on Reddit
fn chain_ranges(seeds: &[Seed], maps: &[Map]) -> Location {
    let mut current: Vec<Range<u64>> = seeds.iter().filter_map(|s| s.as_range()).cloned().collect();

    for map in maps {
        current = current
            .iter()
            .flat_map(|r| map.translate_range(r.clone()))
            .collect();
    }

    let value = current.into_iter().map(|r| r.start).min().unwrap();
    Location(value)
}

fn parse_input_single_seeds(input: &str) -> IResult<&str, (Vec<Seed>, Vec<Map>)> {
    let (input, seeds) = parse_single_seeds(input)?;
    let (input, maps) = parse_maps(input)?;
    Ok((input, (seeds, maps)))
}

fn parse_input_seed_ranges(input: &str) -> IResult<&str, (Vec<Seed>, Vec<Map>)> {
    let (input, seeds) = parse_seed_ranges(input)?;
    let (input, maps) = parse_maps(input)?;
    Ok((input, (seeds, maps)))
}

fn parse_single_seeds(input: &str) -> IResult<&str, Vec<Seed>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, seeds) = many1(preceded(multispace1, digit1))(input)?;
    let seeds = seeds
        .into_iter()
        .flat_map(str::parse)
        .map(Seed::Single)
        .collect();
    Ok((input, seeds))
}

fn parse_one_seed_range(input: &str) -> IResult<&str, Seed> {
    let (input, _) = multispace0(input)?;
    let (input, start) = terminated(digit1, multispace1)(input)?;
    let (input, length) = terminated(digit1, multispace1)(input)?;

    let start: u64 = start.parse().unwrap();
    let length: u64 = length.parse().unwrap();

    Ok((input, Seed::Range(start..start + length)))
}

fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<Seed>> {
    let (input, _) = tag("seeds: ")(input)?;
    many1(parse_one_seed_range)(input)
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    many1(parse_single_map)(input)
}

#[allow(dead_code)]
#[derive(Debug)]
struct MapKind {
    from: Element,
    to: Element,
}

#[derive(Debug, Clone)]
enum Element {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

fn parse_map_element(input: &str) -> IResult<&str, Element> {
    use Element::*;
    alt((
        value(Seed, tag("seed")),
        value(Soil, tag("soil")),
        value(Fertilizer, tag("fertilizer")),
        value(Water, tag("water")),
        value(Light, tag("light")),
        value(Temperature, tag("temperature")),
        value(Humidity, tag("humidity")),
        value(Location, tag("location")),
    ))(input)
}

fn parse_map_type(input: &str) -> IResult<&str, MapKind> {
    let (input, from_type) = parse_map_element(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, to_type) = parse_map_element(input)?;
    Ok((
        input,
        MapKind {
            from: from_type,
            to: to_type,
        },
    ))
}

#[derive(Debug)]
struct MapLine {
    source: Range<u64>,
    dest: Range<u64>,
}

impl MapLine {
    fn new(source_start: u64, dest_start: u64, length: u64) -> MapLine {
        Self {
            source: source_start..source_start + length,
            dest: dest_start..dest_start + length,
        }
    }

    fn in_source(&self, value: u64) -> bool {
        self.source.contains(&value)
    }

    fn translate(&self, value: u64) -> u64 {
        self.dest.start + (value - self.source.start)
    }

    fn try_translate(&self, value: u64) -> Option<u64> {
        if self.in_source(value) {
            Some(self.translate(value))
        } else {
            None
        }
    }

    /// From u/legobmw99 on Reddit
    fn translate_subrange(&self, r: Range<u64>) -> (Option<Range<u64>>, Vec<Range<u64>>) {
        let mapped_start = self.try_translate(r.start);
        let mapped_end = self.try_translate(r.end - 1).map(|v| v + 1);

        let mapped = match (mapped_start, mapped_end) {
            (None, None) => None,
            (None, Some(end)) => Some(self.dest.start..end),
            (Some(start), None) => Some(start..self.dest.end),
            (Some(start), Some(end)) => Some(start..end),
        };

        let mut remaining = Vec::new();
        if mapped.is_some() {
            if self.source.start > r.start {
                remaining.push(r.start..self.source.start);
            }
            if self.source.end < r.end {
                remaining.push(self.source.end..r.end);
            }
        }
        (mapped, remaining)
    }
}

fn parse_map_line(input: &str) -> IResult<&str, MapLine> {
    let (input, dest_range_start) = terminated(digit1, multispace1)(input)?;
    let (input, source_range_start) = terminated(digit1, multispace1)(input)?;
    let (input, range_length) = terminated(digit1, multispace1)(input)?;
    let line = MapLine::new(
        source_range_start.parse().unwrap(),
        dest_range_start.parse().unwrap(),
        range_length.parse().unwrap(),
    );
    Ok((input, line))
}

fn parse_single_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = multispace0(input)?;
    let (input, kind) = parse_map_type(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    let (input, lines) = many1(parse_map_line)(input)?;

    let map = Map { kind, lines };
    Ok((input, map))
}

#[cfg(test)]
mod test {
    use crate::day_05::chain_ranges;

    use super::{
        chain, parse_input_seed_ranges, parse_input_single_seeds, part_one, Location, Seed, INPUT,
    };
    const TEST_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn day5_parse_test_input() {
        let (seeds, maps) = parse_input_single_seeds(TEST_INPUT).unwrap().1;
        assert_eq!(
            &seeds,
            &[
                Seed::Single(79),
                Seed::Single(14),
                Seed::Single(55),
                Seed::Single(13)
            ]
        );
        assert_eq!(maps.len(), 7);
    }

    #[test]
    fn day5_test_translate() {
        let (seeds, maps) = parse_input_single_seeds(TEST_INPUT).unwrap().1;
        let soils: Vec<u64> = seeds
            .into_iter()
            .map(|s| maps[0].translate(*s.as_single().unwrap()))
            .collect();
        assert_eq!(&soils, &[81, 14, 57, 13]);
    }

    #[test]
    fn day5_test_locations() {
        let (seeds, maps) = parse_input_single_seeds(TEST_INPUT).unwrap().1;
        let locations: Vec<Location> = seeds.iter().map(|s| chain(s, &maps)).collect();
        assert_eq!(
            &locations,
            &[Location(82), Location(43), Location(86), Location(35)]
        );
    }

    #[test]
    fn day5_test_min_location() {
        let (seeds, maps) = parse_input_single_seeds(TEST_INPUT).unwrap().1;
        assert_eq!(part_one(&seeds, &maps), 35);
    }

    #[test]
    fn day5_real_min_location() {
        let (seeds, maps) = parse_input_single_seeds(INPUT).unwrap().1;
        assert_eq!(part_one(&seeds, &maps), 486613012);
    }

    #[test]
    fn day5_test_parse_seed_ranges() {
        let (seeds, _) = parse_input_seed_ranges(TEST_INPUT).unwrap().1;
        assert_eq!(&seeds, &[Seed::Range(79..93), Seed::Range(55..68),]);
    }

    #[test]
    fn day5_test_calculate_with_ranges() {
        let (seeds, maps) = parse_input_seed_ranges(TEST_INPUT).unwrap().1;
        let min_location = chain_ranges(&seeds, &maps);
        assert_eq!(min_location.0, 46);
    }
}
