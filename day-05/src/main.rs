use std::collections::BTreeSet;

use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, space1, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, PartialEq)]
struct Conversion {
    src: u64,
    dest: u64,
    offset: u64,
}

impl Conversion {
    fn convert(&self, input: u64) -> Option<u64> {
        if self.src <= input && input < (self.src + self.offset) {
            Some(input - self.src + self.dest)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct AlmanacMap(Vec<Conversion>);

#[derive(Debug, PartialEq)]
struct ValueRange {
    start: u64,
    length: u64,
}

impl AlmanacMap {
    fn convert(&self, source: u64) -> u64 {
        match self
            .0
            .iter()
            .map(|entry| entry.convert(source))
            .find_map(|e| e)
        {
            Some(dest) => dest,
            None => source,
        }
    }

    fn convert_range(&self, range: ValueRange) -> Vec<ValueRange> {
        let mut slices = BTreeSet::new();
        let range_end = range.start + range.length;

        for entry in &self.0 {
            let source_end = entry.src + entry.offset;

            if range_end < entry.src || range.start > source_end {
                continue;
            }

            if entry.src > range.start {
                slices.insert(entry.src);
            }

            if source_end < range_end {
                slices.insert(source_end);
            }
        }
        slices.insert(range_end);

        let mut output = Vec::new();
        let mut current = range.start;

        for position in slices {
            output.push(ValueRange {
                start: self.convert(current),
                length: position - current,
            });
            current = position;
        }

        output
    }
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |value, map| map.convert(value))
    }

    fn seed_ranges(&self) -> impl Iterator<Item = ValueRange> + '_ {
        (0..self.seeds.len()).step_by(2).map(|i| ValueRange {
            start: self.seeds[i],
            length: self.seeds[i + 1],
        })
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, nums) = preceded(tag("seeds: "), separated_list1(tag(" "), u64))(input)?;
    Ok((input, nums))
}

// 50 98 2 -> Conversion
fn parse_conversion(input: &str) -> IResult<&str, Conversion> {
    let (input, nums) = separated_list1(tag(" "), u64)(input)?;

    Ok((
        input,
        Conversion {
            src: *nums.get(1).unwrap(),
            dest: *nums.get(0).unwrap(),
            offset: *nums.get(2).unwrap(),
        },
    ))
}

fn parse_map(input: &str) -> IResult<&str, AlmanacMap> {
    let (input, conversions) = preceded(
        permutation((
            opt(tag("\n\n")),
            alpha1,
            tag("-to-"),
            alpha1,
            space1,
            tag("map:\n"),
        )),
        separated_list1(tag("\n"), parse_conversion),
    )(input)?;

    Ok((input, AlmanacMap(conversions)))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, maps) = separated_list1(tag("\n\n"), parse_map)(input)?;

    Ok((input, Almanac { seeds, maps }))
}

fn task_1(input: &str) -> u64 {
    let almanac = parse_almanac(input).unwrap().1;

    almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap()
}

fn task_2(input: &str) -> u64 {
    let almanac = parse_almanac(input).unwrap().1;

    let mut current: Vec<ValueRange> = almanac.seed_ranges().collect();
    let mut future = Vec::new();

    for map in almanac.maps {
        for range in current {
            future.extend(map.convert_range(range));
        }
        current = future;
        future = Vec::new();
    }

    current.iter().map(|range| range.start).min().unwrap()
}

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    // #[ignore]
    fn task_1_works() {
        let input = "seeds: 79 14 55 13

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

        let result = task_1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn task_2_works() {
        let input = "seeds: 79 14 55 13

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

        let result = task_2(input);
        assert_eq!(result, 46);
    }
}
