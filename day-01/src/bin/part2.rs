#![warn(clippy::pedantic)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut preproceeded_input = input.to_string();

    let number_map: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut i = 0;

    while i < preproceeded_input.len() {
        for (key, value) in &number_map {
            if let Some(next) = preproceeded_input.get(i..i + key.len()) {
                if next == *key {
                    preproceeded_input.replace_range(i..i + key.len(), value);
                    break;
                }
            }
        }

        i += 1;
    }

    let lines: Vec<&str> = preproceeded_input.lines().collect();

    let mut sum: u32 = 0;

    for line in lines {
        let mut digits: Vec<u32> = Vec::new();

        line.chars().for_each(|c| {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit);
            };
        });

        let calibration_value: u32 =
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse()
                .unwrap();

        sum += calibration_value;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
