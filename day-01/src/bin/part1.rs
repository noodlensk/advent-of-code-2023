#![warn(clippy::pedantic)]
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

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
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
