#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

#[derive(Clone)]
struct Range {
    from: usize,
    destination: usize,
    length: usize,
}

impl Range {
    fn in_range(&self, n: usize) -> bool {
        self.from <= n && self.from + self.length > n
        // self.from <= n && self.from + self.length >= n
    }

    fn mapped_value(&self, n: usize) -> Option<usize> {
        if !self.in_range(n) {
            None
        } else {
            Some(n - self.from + self.destination)
        }
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn get_value(&self, n: usize) -> usize {
        for r in &self.ranges {
            if let Some(val) = r.mapped_value(n) {
                return val;
            }
        }
        n
    }
}
fn part2(input: &str) -> String {
    let input_sections: Vec<&str> = input.split("\n\n").collect();

    let (_, seeds_string) = input_sections[0].split_once(':').unwrap();

    let seeds: Vec<usize> = seeds_string
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    dbg!(seeds.clone());
    let mut chain_of_maps: Vec<Map> = Vec::new();

    for &i in input_sections.iter().skip(1) {
        chain_of_maps.push(parse_block(i));
    }

    let mut min: Option<usize> = None;

    for chunk in seeds.chunks(2) {
        for s in chunk[0]..chunk[0] + chunk[1] {
            // dbg!(s);
            if s == usize::MAX {
                dbg!("!!!!!");
            }
            let mut input = s;
            for map in &chain_of_maps {
                let out = map.get_value(input);
                input = out;
            }

            match min {
                Some(curr_value) if curr_value > input => min = Some(input),
                None => min = Some(input),
                _ => {}
            }
        }
    }

    min.unwrap().to_string()
}
fn parse_block(s: &str) -> Map {
    let mut result: Map = Map { ranges: Vec::new() };

    for line in s.lines().skip(1) {
        let numbers: Vec<usize> = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let r = Range {
            from: numbers[1],
            destination: numbers[0],
            length: numbers[2],
        };

        result.ranges.push(r);
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "46");
    }
}
