use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

#[derive(Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    our_numbers: Vec<usize>,
}

impl Card {
    fn from(s: &str) -> Self {
        let (winning_part, our_part) = s.split_once('|').unwrap();

        let (card_name_part, winning_part) = winning_part.split_once(':').unwrap();

        let winning_numbers: Vec<usize> = winning_part
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let our_numbers: Vec<usize> = our_part
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let (_, card_id) = card_name_part.trim().split_once(' ').unwrap();

        Card {
            id: card_id.trim().parse::<usize>().unwrap(),
            winning_numbers,
            our_numbers,
        }
    }

    fn matching_numbers(&self) -> Vec<usize> {
        let set1: HashSet<_> = self.winning_numbers.iter().cloned().collect();

        self.our_numbers
            .iter()
            .cloned()
            .filter(|num| set1.contains(num))
            .collect::<Vec<usize>>()
    }
}
fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let card = Card::from(line);

        cards.push(card);
    }

    let mut cards_cnt: HashMap<usize, usize> = HashMap::new();
    let mut cards_queue = cards.clone();

    while let Some(card) = cards_queue.pop() {
        let entry = cards_cnt.entry(card.id).or_default();
        *entry += 1;

        let matching_cnt = card.matching_numbers().len();

        if matching_cnt == 0 {
            continue;
        }

        // if card.id >= cards.len() - 1 {
        //     continue;
        // }

        let start_idx = card.id;
        let end_idx = (start_idx + matching_cnt).min(cards.len());

        dbg!((
            cards_queue.len(),
            card.id,
            card.matching_numbers(),
            start_idx,
            end_idx
        ));

        // for i in start_idx..end_idx {
        //     cards_queue.push(cards.get(i).unwrap().clone());
        //     // dbg!((card.id, i));
        // }

        let mut index = card.id;
        for _ in 0..matching_cnt {
            if index >= cards.len() {
                break;
            }
            let next_card = cards.get(index).unwrap().clone();
            cards_queue.push(next_card);
            index += 1;
        }
    }

    // dbg!(cards_cnt.clone());

    format!("{}", cards_cnt.values().sum::<usize>())
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let card = Card::from(line);
        cards.push(card);
    }

    // Map to hold card id and count
    let mut counts_map: HashMap<usize, usize> = HashMap::new();

    // Initial count for each card is 1
    for i in 0..cards.len() {
        counts_map.insert(i, 1);
    }

    for i in 0..cards.len() {
        let card = &cards[i];
        let count = counts_map[&i];
        // get the count of matching numbers
        let matching_count = card.matching_numbers().len();

        for j in 1..=matching_count {
            // if it's a valid card index
            if cards.get(i + j).is_some() {
                *counts_map.entry(i + j).or_insert(0) += count;
            } else {
                break;
            }
        }
    }

    counts_map.values().sum::<usize>().to_string()
}
fn part3(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        let card = Card::from(line);
        cards.push(card);
    }

    // Use a vector to handle card counts
    let mut card_counts: Vec<usize> = vec![0; cards.len()];

    let mut current_card_idx = 0;

    while current_card_idx < cards.len() {
        // Increment count for current card
        card_counts[current_card_idx] += 1;

        // Count of next cards to increment based on matched numbers of current card
        let matched_count = cards[current_card_idx].matching_numbers().len();

        // Increment count for matched_count number of next cards
        for next_card_idx in 1..=matched_count {
            if current_card_idx + next_card_idx < cards.len() {
                card_counts[current_card_idx + next_card_idx] += 1;
            }
        }

        current_card_idx += 1;
    }

    format!("{}", card_counts.iter().sum::<usize>())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30");
    }
}
