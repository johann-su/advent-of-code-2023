use std::{vec, collections::HashMap};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, multispace1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Clone)]
struct Node {
    value: String,
    l: String,
    r: String,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    map_res(anychar, |c| match c {
        'R' => Ok(Direction::R),
        'L' => Ok(Direction::L),
        _ => Err("Invalid character"),
    })(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = many1(parse_direction)(input)?;

    Ok((input, directions))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, node) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)?;

    Ok((
        input,
        Node {
            value: node.0.to_owned(),
            l: node.1 .0.to_owned(),
            r: node.1 .1.to_owned(),
        },
    ))
}

fn parse_tree(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, nodes) = preceded(multispace1, separated_list1(multispace1, parse_node))(input)?;

    Ok((input, nodes))
}

fn task_1(input: &str) -> u32 {
    let (input, directions) = parse_directions(input).unwrap();
    let tree = parse_tree(input).unwrap().1;

    let mut node: &Node = tree.iter().find(|n| n.value == "AAA").unwrap();
    let mut i = 0;

    while node.value != "ZZZ" {
        let direction = directions.get(i % directions.len()).unwrap();
        match direction {
            Direction::R => node = tree.iter().find(|n| n.value == node.r).unwrap(),
            Direction::L => node = tree.iter().find(|n| n.value == node.l).unwrap(),
        }
        i += 1;
    }

    i as u32
}

fn prime_factors(number: u64) -> HashMap<u64, u32> {
    let mut n = number;
    let mut factors: HashMap<u64, u32> = HashMap::new();

    while n%2 == 0 {
        n /= 2;
        if let Some(i) = factors.get_mut(&2) {
            *i+=1;
        } else {
            factors.insert(2, 1);
        }
    }

    for i in (3..(n as f64).sqrt() as u64).skip(2) {
        while n%i == 0 {  
            n = n/i;
            if let Some(j) = factors.get_mut(&i) {
                *j+=1;
            } else {
                factors.insert(i, 1);
            } 
        }  
    }

    if n > 2 {
        factors.insert(n, 1);
    }

    factors
}

fn lcm(inp: Vec<u64>) -> u64 {
    let mut biggest_prime_factors: Vec<(u64, u32)> = vec![];
    
    for n in inp.iter() {
        let factors = prime_factors(*n);
        for factor in factors {
            if let Some(f) = biggest_prime_factors.iter_mut().find(|f| f.0 == factor.0) {
                if f.1 < factor.1 as u32 {
                    f.1 = factor.1 as u32;
                }
            } else {
                biggest_prime_factors.push((factor.0, factor.1 as u32));
            }
        }
    }

    biggest_prime_factors.iter().map(|n| n.0.pow(n.1)).product()
}

fn task_2(input: &str) -> u64 {
    let (input, directions) = parse_directions(input).unwrap();
    let tree = parse_tree(input).unwrap().1;

    let nodes: Vec<&Node> = tree.iter().filter(|n| n.value.ends_with("A")).collect_vec();
    
    let zs = nodes.iter().map(|node| {
        let mut i = 0;
        let mut curr_node: &Node = node;

        while !curr_node.value.ends_with("Z") {
            let direction = directions.get(i % directions.len()).unwrap();
            match direction {
                Direction::R => curr_node = tree.iter().find(|n| n.value == curr_node.r).unwrap(),
                Direction::L => curr_node = tree.iter().find(|n| n.value == curr_node.l).unwrap(),
            }
            i += 1; 
        }

        i as u64
    }).collect_vec();

    lcm(zs) as u64
}

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_1_works() {
        {
            let input = "RL
    
AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

            let result = task_1(input);
            assert_eq!(result, 2);
        }

        {
            let input = "LLR
    
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

            let result = task_1(input);
            assert_eq!(result, 6);
        }
    }

    #[test]
    fn task_2_works() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let result = task_2(input);
        assert_eq!(result, 6);
    }
}
