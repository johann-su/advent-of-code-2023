use nom::{
    branch::permutation,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::map_res,
    combinator::opt,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
struct Card {
    instances: u32,
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>,
}

impl Card {
    fn get_matches(&self) -> Vec<u32> {
        self.actual_numbers
            .iter()
            .filter_map(|&i| {
                if self.winning_numbers.contains(&i) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_value(&self) -> u32 {
        self.get_matches().iter().fold(0, |mut acc, _| {
            if acc == 0 {
                acc = 1
            } else {
                acc *= 2;
            }
            acc
        })
    }
}

fn to_u32(input: &str) -> IResult<&str, u32> {
    map_res(preceded(opt(space1), digit1), str::parse)(input)
}

// 41 48 83 86 17 -> Vec<u32>
fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, numbers) = separated_list1(tag(" "), to_u32)(input)?;

    Ok((input, numbers))
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53 -> Card
fn parse_card(input: &str) -> IResult<&str, Card> {
    // let (input, id) = preceded(permutation((tag("Card"), space1)), to_u32)(input)?;
    let (input, numbers) = preceded(
        permutation((alpha1, space1, digit1, tag(":"))),
        separated_pair(numbers, tag(" | "), numbers),
    )(input)?;

    Ok((
        input,
        Card {
            instances: 1,
            winning_numbers: numbers.0.to_owned(),
            actual_numbers: numbers.1.to_owned(),
        },
    ))
}

fn task_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| parse_card(l).expect("line should be a valid card").1)
        .fold(0, |mut acc, i| {
            acc += i.get_value();
            acc
        })
}

fn task_2(input: &str) -> u32 {
    let cards: Vec<_> = input
        .lines()
        .map(|l| parse_card(l).expect("line should be a valid card").1)
        .collect();

    let mut cards_new = cards.clone();

    for (i, card) in cards.iter().enumerate() {
        let matches = card.get_matches();

        for j in 1..=matches.len() {
            let card_instances = cards_new.get(i).expect("should be valid index").instances;
            cards_new
                .get_mut(i + j)
                .expect("should be valid index")
                .instances += card_instances
        }
    }

    cards_new
        .iter()
        .fold(0, |mut acc, i| {
            acc += i.instances;
            acc
        })
}

fn main() {
    let input = include_str!("./data.txt");

    println!("task 1: {}", task_1(input));
    println!("task 2: {}", task_2(input));
}

#[cfg(test)]
mod tests {
    use crate::{task_2, task_1};

    #[test]
    fn task_1_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = task_1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn task_2_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = task_2(input);
        assert_eq!(result, 30);
    }
}
