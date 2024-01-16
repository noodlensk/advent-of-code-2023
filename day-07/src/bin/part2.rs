#![warn(clippy::pedantic)]
#![allow(dead_code)]
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, PartialOrd)]
enum Card {
    A,
    K,
    Q,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    J,
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

    fn hand_type2(&self) -> Option<HandType> {
        let mut idx: HashMap<Card, usize> = HashMap::new();

        let joker_cnt = self.cards.iter().filter(|&c| *c == Card::J).count();
        for c in &self.cards {
            if c == &Card::J {
                continue;
            }

            *idx.entry(*c).or_insert(0) += 1;
        }

        let max_eq_cards_cnt = idx.values().max().unwrap_or(&0);

        match max_eq_cards_cnt + joker_cnt {
            5 => Some(HandType::FiveOfKind),
            4 => Some(HandType::FourOfKind),
            3 if idx.values().filter(|&v| *v == 2).count() > 0 => Some(HandType::FullHouse),
            3 => Some(HandType::ThreeOfKind),
            2 if idx.values().filter(|&v| *v == 2).count() > 1 => Some(HandType::TwoPair),
            2 => Some(HandType::OnePair),
            1 => Some(HandType::HighCard),
            _ => None,
        }
    }
    fn hand_type(&self) -> Option<HandType> {
        let mut idx: HashMap<Card, usize> = HashMap::new();

        let joker_cnt = self.cards.iter().filter(|&c| *c == Card::J).count();

        for c in &self.cards {
            if c == &Card::J {
                continue;
            }
            *idx.entry(*c).or_insert(0) += 1;
        }

        // Step 1: Extract and sort the counts
        let mut counts: Vec<_> = idx.values().cloned().collect();
        counts.sort_unstable();

        // Step 2: Create a tuple with (max_count, second_max_count), considering jokers
        let type_count = match counts.as_slice() {
            [] => (0, 0),
            [single] => (*single, 0),
            [rest @ .., highest] => {
                let second_highest = rest.last().unwrap_or(&0).clone();
                (*highest, second_highest)
            }
        };

        // Step 3: Adjust for joker counts
        let type_count = if joker_cnt > 0 {
            (type_count.0 + joker_cnt, type_count.1 + joker_cnt.min(1))
        } else {
            type_count
        };

        // Step 4: Determine hand type
        match type_count {
            (5, _) => Some(HandType::FiveOfKind),
            (4, _) => Some(HandType::FourOfKind),
            (3, 2) => Some(HandType::FullHouse),
            (3, _) => Some(HandType::ThreeOfKind),
            (_, _) if counts.iter().filter(|&&v| v == 2).count() > 1 => Some(HandType::TwoPair),
            (2, _) => Some(HandType::OnePair),
            (1, _) => Some(HandType::HighCard),
            _ => None,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (s, o) = (self.hand_type().unwrap(), other.hand_type().unwrap());

        let res = s.cmp(&o);

        // dbg!(self.cards.clone(), other.cards.clone(), s, o, res);
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
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
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
        // dbg!(i, val);
        sum += val.bid * (i + 1usize);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "5905");
    }
}
