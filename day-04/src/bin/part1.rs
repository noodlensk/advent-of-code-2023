use std::collections::HashSet;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;

    for line in lines {
        let (mut winning_part, our_part) = line.split_once('|').unwrap();

        (_, winning_part) = winning_part.split_once(':').unwrap();

        let winning_numbers: Vec<usize> = winning_part
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let our_numbers: Vec<usize> = our_part
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let intersect = intersection(&winning_numbers, &our_numbers);

        if !intersect.is_empty() {
            let points = 2u32.pow((intersect.len() - 1) as u32);
            sum += points;
        }
    }

    sum.to_string()
}

fn intersection(vec1: &[usize], vec2: &[usize]) -> Vec<usize> {
    let set1: HashSet<_> = vec1.iter().cloned().collect();
    let set2: HashSet<_> = vec2.iter().cloned().collect();
    set1.intersection(&set2).cloned().collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "13");
    }
}
