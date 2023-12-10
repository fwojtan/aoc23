use std::{cmp, slice::Iter};

use counter::Counter;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day07;

impl Solution for Day07 {
    type ParsedInput = (Vec<Hand>, Vec<Hand>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut joker_hands = Vec::new();
        let mut counter = Counter::new();
        let mut counter2 = Counter::new();
        let hands = input_lines
            .lines()
            .map(|line| {
                let mut parts = line.split_ascii_whitespace();
                let hand = parts.next().unwrap();
                let bid = parts.next().unwrap().parse::<u16>().unwrap();
                joker_hands.push(Hand::new_jokers(hand, bid, &mut counter2));
                Hand::new(hand, bid, &mut counter)
            })
            .collect::<Vec<_>>();
        (hands, joker_hands)
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        solve(&mut parsed_input.0)
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        solve(&mut parsed_input.1)
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 5],
    bid: u16,
    hand_type: HandType,
}

impl Hand {
    fn new(hand_str: &str, bid: u16, counter: &mut Counter<char, usize>) -> Self {
        let mut cards = hand_str.chars().map(|c| Card::from_char(&c).unwrap());
        Hand {
            cards: [
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
            ],
            bid,
            hand_type: Hand::hand_type(hand_str, counter),
        }
    }

    fn hand_type(hand_str: &str, counter: &mut Counter<char, usize>) -> HandType {
        counter.clear();
        counter.update(hand_str.chars());
        let count = counter.most_common();
        let mut count_iter = count.iter();
        HandType::from_count(count_iter.next().unwrap().1, &mut count_iter)
    }

    fn new_jokers(hand_str: &str, bid: u16, counter: &mut Counter<char, usize>) -> Self {
        let mut cards = hand_str
            .chars()
            .map(|c| Card::from_char_jokers(&c).unwrap());
        Hand {
            cards: [
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
            ],
            bid,
            hand_type: Hand::hand_type_jokers(hand_str, counter),
        }
    }

    fn hand_type_jokers(hand_str: &str, counter: &mut Counter<char, usize>) -> HandType {
        counter.clear();
        counter.update(hand_str.chars());
        let n_jokers = counter.remove(&'J').unwrap_or(0);
        let count = counter.most_common();
        let mut count_iter = count.iter();
        HandType::from_count(
            count_iter.next().unwrap_or(&('_', 0)).1 + n_jokers,
            &mut count_iter,
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
    Joker,
}

impl Card {
    fn from_char(c: &char) -> Result<Self, ()> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            '1' => Ok(Self::One),
            _ => Err(()),
        }
    }

    fn from_char_jokers(c: &char) -> Result<Self, ()> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            '1' => Ok(Self::One),
            'J' => Ok(Self::Joker),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_count(most_common: usize, count_iter: &mut Iter<'_, (char, usize)>) -> HandType {
        match most_common {
            2 => {
                if count_iter.next().unwrap().1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            3 => {
                if count_iter.next().unwrap().1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => HandType::HighCard,
        }
    }
}

fn solve(input: &mut [Hand]) -> String {
    input.sort_by(
        |hand_a, hand_b| match hand_a.hand_type.cmp(&hand_b.hand_type) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Equal => {
                let mut ordering = cmp::Ordering::Equal;
                for (card_a, card_b) in hand_a.cards.iter().zip(hand_b.cards.iter()) {
                    match card_a.cmp(card_b) {
                        cmp::Ordering::Less => {
                            ordering = cmp::Ordering::Less;
                            break;
                        }
                        cmp::Ordering::Equal => (),
                        cmp::Ordering::Greater => {
                            ordering = cmp::Ordering::Greater;
                            break;
                        }
                    }
                }
                ordering
            }
            cmp::Ordering::Greater => cmp::Ordering::Greater,
        },
    );
    input
        .iter_mut()
        .rev()
        .enumerate()
        .map(|(i, hand)| hand.bid as u64 * (i + 1) as u64)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(
            Day07::solve_part_one(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            "6440".to_string()
        )
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(Day07::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(Day07::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
