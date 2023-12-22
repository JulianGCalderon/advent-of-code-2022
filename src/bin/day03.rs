use itertools::Itertools;
use std::collections::HashSet;

fn part1(data: String) -> usize {
    data.lines().map(error_priority).sum()
}

fn error_priority(line: &str) -> usize {
    let middle = line.len() / 2 as usize;
    let (first, second) = line.split_at(middle);

    let first = first.bytes().collect::<HashSet<u8>>();
    let second = second.bytes().collect::<HashSet<u8>>();

    let priority_sum = first.intersection(&second).map(get_priority).sum();

    priority_sum
}

fn get_priority(item: &u8) -> usize {
    match item {
        b'a'..=b'z' => (*item - b'a') as usize + 1,
        b'A'..=b'Z' => (*item - b'A') as usize + 27,
        _ => panic!("Item was not ascii alphabet character"),
    }
}

fn part2(data: String) -> usize {
    data.lines()
        .chunks(3)
        .into_iter()
        .map(Itertools::collect_vec)
        .map(badge_priority)
        .sum()
}

fn badge_priority(group: Vec<&str>) -> usize {
    let result = group
        .into_iter()
        .map(to_set)
        .reduce(intersect)
        .expect("Group was empty")
        .iter()
        .map(get_priority)
        .sum();

    result
}

fn intersect(a: HashSet<u8>, b: HashSet<u8>) -> HashSet<u8> {
    a.intersection(&b).map(ToOwned::to_owned).collect()
}

fn to_set(line: &str) -> HashSet<u8> {
    line.bytes().collect()
}

aoc::main!(3);
