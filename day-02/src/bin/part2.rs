#![warn(clippy::pedantic)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);

    dbg!(output);
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Unknown,
}

#[derive(Debug, Clone)]
struct Game {
    rounds: Vec<HashMap<Color, u32>>,
}

impl Game {
    fn min_possible_power(&self) -> u32 {
        let mut m: HashMap<Color, u32> = HashMap::new();

        for r in &self.rounds {
            for (color, n) in r {
                let count = m.entry(*color).or_insert(0);

                if n > count {
                    *count = *n;
                }
            }
        }

        let mut power: u32 = 1;

        for n in m.values() {
            power *= n;
        }

        power
    }
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;

    for line in lines {
        let striped_line = line.strip_prefix("Game ").unwrap();

        let (_, rounds) = striped_line.split_once(':').unwrap();

        let mut g = Game { rounds: Vec::new() };

        for round_str in rounds.trim().split(';') {
            let mut round: HashMap<Color, u32> = HashMap::new();

            for cube in round_str.trim().split(',') {
                let (number_raw, color_raw) = cube.trim().split_once(' ').unwrap();

                let color = match color_raw {
                    "red" => Color::Red,
                    "blue" => Color::Blue,
                    "green" => Color::Green,
                    _ => Color::Unknown,
                };

                round.insert(color, number_raw.parse::<u32>().unwrap());
            }

            g.rounds.push(round);
        }

        dbg!(line, g.min_possible_power());

        sum += g.min_possible_power();
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}
