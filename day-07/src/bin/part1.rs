#![warn(clippy::pedantic)]
#![allow(dead_code)]
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn from_str(s: &str) -> Self {
        let (cards_string, bid_str) = s.split_once(' ').unwrap();

        let mut cards: Vec<Card> = Vec::new();

        for c in cards_string.chars() {
            let card = match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::N9,
                '8' => Card::N8,
                '7' => Card::N7,
                '6' => Card::N6,
                '5' => Card::N5,
                '4' => Card::N4,
                '3' => Card::N3,
                '2' => Card::N2,
                _ => panic!("unknown cards"),
            };

            cards.push(card);
        }

        Hand {
            cards,
            bid: bid_str.parse().unwrap(),
        }
    }

    fn hand_type(&self) -> Option<HandType> {
        let mut idx: HashMap<Card, usize> = HashMap::new();

        for c in &self.cards {
            *idx.entry(*c).or_insert(0) += 1;
        }

        match idx.values().max() {
            Some(&5) => Some(HandType::FiveOfKind),
            Some(&4) => Some(HandType::FourOfKind),
            Some(&3) if idx.values().filter(|&v| *v == 2).count() > 0 => Some(HandType::FullHouse),
            Some(&3) => Some(HandType::ThreeOfKind),
            Some(&2) if idx.values().filter(|&v| *v == 2).count() > 1 => Some(HandType::TwoPair),
            Some(&2) => Some(HandType::OnePair),
            Some(&1) => Some(HandType::HighCard),
            _ => None,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (s, o) = (self.hand_type().unwrap(), other.hand_type().unwrap());

        let res = s.cmp(&o);

        dbg!(self.cards.clone(), other.cards.clone(), s, o, res);
        if res == Ordering::Equal {
            // dbg!(self, other);
            for i in 0..5 {
                let cmpr_cards = self.cards[i].partial_cmp(&other.cards[i]).unwrap();

                if cmpr_cards != Ordering::Equal {
                    // dbg!(cmpr_cards);
                    return cmpr_cards;
                }
            }
        }

        res
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let (s, o) = (self.hand_type().unwrap(), other.hand_type().unwrap());
        s == o
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut hands: Vec<Hand> = Vec::new();

    for &l in &lines {
        let h = Hand::from_str(l);

        hands.push(h);
    }

    hands.sort();
    hands.reverse();

    let mut sum: usize = 0;

    for (i, val) in hands.iter().enumerate() {
        dbg!(i, val);
        sum += val.bid * (i + 1usize);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "6440");
    }
}

pub fn some
