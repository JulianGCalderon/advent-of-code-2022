use itertools::Itertools;

fn part1(data: String) -> usize {
    first_marker(data, 4)
}

fn first_marker(data: String, window_size: usize) -> usize {
    data.chars()
        .collect_vec()
        .windows(window_size)
        .find_position(|window| window.into_iter().duplicates().count() == 0)
        .expect("There should be marker")
        .0
        + window_size
}

fn part2(data: String) -> usize {
    first_marker(data, 14)
}

aoc::main!(6);
