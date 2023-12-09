use itertools::Itertools;
use std::ops::Range;

use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, space1, u64},
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Conversion {
    src: u64,
    dest: u64,
    offset: u64,
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

fn parse_map(input: &str) -> IResult<&str, Vec<Conversion>> {
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

    Ok((input, conversions))
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Vec<Conversion>>> {
    let (input, convs) = separated_list1(tag("\n\n"), parse_map)(input)?;

    Ok((input, convs))
}

fn task_1(input: &str) -> u64 {
    let (input, mut seeds) = parse_seeds(input).unwrap();
    let maps = parse_maps(input).unwrap().1;

    for conv_map in maps {
        for s in seeds.iter_mut() {
            for conv in conv_map.iter() {
                if conv.src as u64 <= *s && conv.src + conv.offset >= *s {
                    *s = (*s - conv.src) + conv.dest;
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

fn parse_seeds_tuple(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, tuples) = preceded(
        tag("seeds: "),
        separated_list1(tag(" "), separated_pair(u64, tag(" "), u64)),
    )(input)?;

    Ok((input, tuples.iter().map(|e| e.0..(e.0 + e.1)).collect()))
}

fn task_2(input: &str) -> u64 {
    let (input, seeds) = parse_seeds_tuple(input).unwrap();
    let maps = parse_maps(input).unwrap().1;

    seeds
        .iter()
        .map(|seeds_range| {
            let mut from: Vec<Range<u64>> = vec![seeds_range.to_owned()];

            for conv_map in maps.iter() {
                let mut to: Vec<Range<u64>> = Vec::new();
                
                for seeds_range in from.iter() {
                    for conv in conv_map.iter() {
                        let diff = conv.dest as i64 - conv.src as i64;

                        // start of seed range is in conv range
                        if seeds_range.start >= conv.src
                            && seeds_range.start <= conv.src + conv.offset
                        {
                            let s_conv = (seeds_range.start as i64 + diff) as u64;

                            // end of seed range is in conv range
                            if seeds_range.end >= conv.src
                                && seeds_range.end <= conv.src + conv.offset
                            {
                                let e_conv = (seeds_range.end as i64 + diff) as u64;
                                to.push(s_conv..e_conv);
                                // break;
                            }
                            // end of seed range is not in conv range
                            else {
                                let e_conv = conv.dest + conv.offset;
                                to.push(s_conv..e_conv);
                                to.push((conv.src + conv.offset)..seeds_range.end);
                                // break;
                            }
                        }
                        // start of seeds is not in conv range
                        else {
                            let s_conv = conv.dest;

                            // end of seeds range is in conv range
                            if seeds_range.end >= conv.src
                                && seeds_range.end <= conv.src + conv.offset
                            {
                                let e_conv = (seeds_range.end as i64 + diff) as u64;
                                to.push(seeds_range.start..conv.src);
                                to.push(s_conv..e_conv);
                                // break;
                            }
                            // end of seed range is not in conv range
                            else {
                                // seeds range overlaps conv range
                                if seeds_range.start < conv.src
                                    && seeds_range.end > conv.src + conv.offset
                                {
                                    to.push(seeds_range.start..conv.src);
                                    to.push(conv.dest..(conv.dest + conv.offset));
                                    to.push((conv.src + conv.offset)..seeds_range.end);
                                    // break;
                                } else {
                                    // to.push(seeds_range.start..seeds_range.end);
                                    // break;
                                }
                            }
                        }
                    }
                }

                if to.len() == 0 {
                    to = from;
                }

                from = to;
            }

            from.iter().map(|i| i.start).min().unwrap()
        })
        .min()
        .unwrap()
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
