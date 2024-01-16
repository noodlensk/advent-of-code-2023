#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let times: Vec<usize> = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    let distances: Vec<usize> = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    // dbg!(times, distances);

    let mut results: Vec<usize> = Vec::new();

    for (t, d) in times.iter().zip(distances.iter()) {
        let number_of_ways: usize = (1..*t).filter(|&k| k * (*t - k) > *d).count();
        results.push(number_of_ways);
    }

    results.iter().product::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "288");
    }
}
