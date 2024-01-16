#![warn(clippy::pedantic)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;

    let mut numbers_matrix: HashMap<usize, HashMap<usize, u32>> = HashMap::new();

    for (i, &line) in lines.iter().enumerate() {
        let mut current_number_str = String::new();
        let mut current_number_start_idx: Option<usize> = None;

        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                current_number_str.push(c);

                if current_number_start_idx.is_none() {
                    current_number_start_idx = Some(j);
                }
            }

            if current_number_start_idx.is_some() && (!c.is_ascii_digit() || j == line.len() - 1) {
                let e = numbers_matrix.entry(i).or_default();

                e.insert(
                    current_number_start_idx.take().unwrap(),
                    current_number_str.parse::<u32>().unwrap(),
                );

                current_number_str.clear();
            }
        }
    }

    let max_x = lines.len();
    let max_y = lines.last().unwrap().len();

    for (&i, v) in &numbers_matrix {
        for (&j, &v2) in v {
            for (x, y) in coordinates_to_around(i, j, format!("{v2}").len(), max_x, max_y) {
                let val = lines[x].chars().nth(y).unwrap();

                if val != '.' && !val.is_ascii_digit() {
                    sum += v2;

                    break;
                }
            }
        }
    }

    sum.to_string()
}

// i, j - coordinates of word start. len - length of word. max_i, max_j - max size of matrix field(starts from 0).
// function need to return all coordinates around word with a distance of 1 cell (including diagonal)
fn coordinates_to_around(
    i: usize,
    j: usize,
    len: usize,
    max_i: usize,
    max_j: usize,
) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();

    let start_i = i.saturating_sub(1);
    let end_i = (i + 1).min(max_i - 1);
    let start_j = j.saturating_sub(1);
    let end_j = (j + len).min(max_j - 1);

    for row in start_i..=end_i {
        for col in start_j..=end_j {
            if !(row == i && col >= j && col < j + len) {
                res.push((row, col));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361");
    }
}
