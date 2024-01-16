#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("");
        assert_eq!(result, "");
    }
}
