use std::ops::RangeInclusive;

use regex::Regex;

type Pair = (RangeInclusive<usize>, RangeInclusive<usize>);

fn part1(data: String) -> usize {
    data.lines()
        .into_iter()
        .map(parse_ranges)
        .filter(has_containment)
        .count()
}

fn parse_ranges(pair: &str) -> Pair {
    let mut ranges = pair.split(",").map(parse_range);

    let left = ranges.next().expect("Input was malformed");
    let right = ranges.next().expect("Input was malformed");

    (left, right)
}

fn parse_range(range: &str) -> RangeInclusive<usize> {
    let regex = Regex::new(r"(\d*)-(\d*)").expect("Regex should be valid");

    let (_, [left, right]) = regex
        .captures(range)
        .expect("Input was malformed")
        .extract();

    let left = left.parse().unwrap();
    let right = right.parse().unwrap();

    left..=right
}

fn has_containment(pair: &Pair) -> bool {}

fn part2(data: String) -> usize {
    let _ = data;

    0
}

aoc::main!(4);
