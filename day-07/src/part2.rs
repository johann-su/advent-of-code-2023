use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, u32},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord)]
struct Hand {
    cards: Vec<u32>,
    t: HandType,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.t != other.t {
            self.t.partial_cmp(&other.t)
        } else {
            if let Some((i, card)) = self
                .cards
                .iter()
                .enumerate()
                .skip_while(|(i, card)| {
                    let other_card = other.cards.get(*i).unwrap();
                    *card == other_card
                })
                .next()
            {
                let other_card = other.cards.get(i).unwrap();
                Some(card.cmp(other_card))
            } else {
                None
            }
        }
    }
}

fn parse_card(input: &str) -> IResult<&str, u32> {
    map_res(anychar, |input| match input {
        'T' => Ok(10),
        'J' => Ok(1),
        'Q' => Ok(11),
        'K' => Ok(12),
        'A' => Ok(13),
        s => s.to_string().parse::<u32>(),
    })(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, cards) = many1(parse_card)(input)?;

    Ok((input, cards))
}

fn get_hand_type(cards: Vec<u32>) -> HandType {
    let mut counts = cards.iter().counts().into_values().collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));

    if let &[5] = &*counts {
        HandType::FiveOfAKind
    } else if let &[4, ..] = &*counts {
        HandType::FourOfAKind
    } else if let &[3, 2] = &*counts {
        HandType::FullHouse
    } else if let &[3, ..] = &*counts {
        HandType::ThreeOfAKind
    } else if let &[2, 2, ..] = &*counts {
        HandType::TwoPair
    } else if let &[2, ..] = &*counts {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, hand_tuple) = separated_pair(parse_cards, tag(" "), u32)(input)?;

    let hand_type: HandType = {
        if hand_tuple.0.contains(&1) {
            (1..14)
                .map(|i| {
                    let mut new_vec = hand_tuple.0.clone();

                    // if there are multiple J it should always be best to make them all the same type
                    for e in new_vec.iter_mut().filter(|x| **x == 1) {
                        *e = i;
                    }

                    get_hand_type(new_vec)
                })
                .max()
                .unwrap()
        } else {
            get_hand_type(hand_tuple.0.clone())
        }
    };

    Ok((
        input,
        Hand {
            bid: hand_tuple.1,
            cards: hand_tuple.0,
            t: hand_type,
        },
    ))
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = separated_list1(newline, parse_hand)(input)?;

    Ok((input, hands))
}

pub fn task_2(input: &str) -> u32 {
    let mut hands = parse_hands(input).unwrap().1;
    hands.sort();

    dbg!(hands.to_owned());

    hands
        .iter()
        .enumerate()
        .map(|(i, card)| (i + 1) as u32 * card.bid)
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn task_2_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = super::task_2(input);
        assert_eq!(result, 5905);
    }
}
