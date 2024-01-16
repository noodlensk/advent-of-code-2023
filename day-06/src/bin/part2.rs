#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let times: usize = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .expect("Failed to parse string as number");

    let distances: usize = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .expect("Failed to parse string as number");
    // dbg!(times, distances);

    (1..times)
        .filter(|&k| k * (times - k) > distances)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "71503");
    }
}
