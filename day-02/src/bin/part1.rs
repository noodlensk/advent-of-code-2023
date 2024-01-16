#![warn(clippy::pedantic)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
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
    id: u32,
    rounds: Vec<HashMap<Color, u32>>,
}

impl Game {
    fn is_possible(&self) -> bool {
        let max_allowed = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)];

        self.rounds.iter().all(|round| {
            round.iter().all(|(color, number)| {
                max_allowed.iter().all(|(max_color, max_number)| {
                    if color == max_color {
                        return number <= max_number;
                    }
                    true
                })
            })
        })
    }
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;

    for line in lines {
        let striped_line = line.strip_prefix("Game ").unwrap();

        let (game_id_raw, rounds) = striped_line.split_once(':').unwrap();

        let game_id = game_id_raw.parse::<u32>().unwrap();

        let mut g = Game {
            id: game_id,
            rounds: Vec::new(),
        };

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

        dbg!(line, g.clone().is_possible(), g.clone());

        if g.is_possible() {
            sum += g.id;
        }
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
        assert_eq!(result, "8");
    }
}
