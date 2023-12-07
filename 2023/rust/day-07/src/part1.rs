use std::ops::Deref;

use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    score: (u32, u32, u32, u32, u32),
    hand_type: HandType,
    bid_amount: u32,
}

fn hand(input: &str) -> IResult<&str, Hand> {
    separated_pair(alphanumeric1, space1, alphanumeric1)
        .map(|(hand, bid)| Hand {
            score: parse_hand_score(hand),
            hand_type: parse_hand_type(hand),
            bid_amount: bid.parse::<u32>().expect("a number"),
        })
        .parse(input)
}

fn parse_hand_score(hand: &str) -> (u32, u32, u32, u32, u32) {
    hand.chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap()
}

fn parse_hand_type(hand: &str) -> HandType {
    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");

    match values.deref() {
        "5" => HandType::FiveOfAKind,
        "14" => HandType::FourOfAKind,
        "23" => HandType::FullHouse,
        "113" => HandType::ThreeOfAKind,
        "122" => HandType::TwoPair,
        "1112" => HandType::OnePair,
        "11111" => HandType::HighCard,
        value => panic!("should never happen. Encountered `{}`", value),
    }
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(newline, hand)(input)
}

pub fn process(input: &str) -> u32 {
    let (_, hands) = parse_hands(input).expect("parsing");

    hands
        .iter()
        .sorted_by_key(|hand| (hand.hand_type as u8, hand.score))
        .enumerate()
        .map(|(index, hand)| hand.bid_amount * (index + 1) as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input));
    }
}
