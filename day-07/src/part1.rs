use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32, anychar},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, Ord)]
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
        'J' => Ok(11),
        'Q' => Ok(12),
        'K' => Ok(13),
        'A' => Ok(14),
        s => s.to_string().parse::<u32>(),
    })(input)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, cards) = many1(parse_card)(input)?;

    Ok((input, cards))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, hand_tuple) = separated_pair(parse_cards, tag(" "), u32)(input)?;

    let hand_type: HandType = {
        let mut cards = hand_tuple.0.iter().counts().into_values().collect::<Vec<_>>();
        cards.sort_by(|a,b| b.cmp(a));

        if let &[5] = &*cards {
            HandType::FiveOfAKind
        } else if let &[4,..] = &*cards {
            HandType::FourOfAKind
        } else if let &[3,2] = &*cards {
            HandType::FullHouse
        } else if let &[3,..] = &*cards {
            HandType::ThreeOfAKind
        } else if let &[2,2,..] = &*cards {
            HandType::TwoPair
        } else if let &[2,..] = &*cards {
            HandType::OnePair
        } else {
            HandType::HighCard
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

pub fn task_1(input: &str) -> u32 {
    let mut hands = parse_hands(input).unwrap().1;
    hands.sort();
    
    hands.iter().enumerate().map(|(i, card)| (i+1) as u32*card.bid).sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn task_1_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = super::task_1(input);
        assert_eq!(result, 6440);
    }
}