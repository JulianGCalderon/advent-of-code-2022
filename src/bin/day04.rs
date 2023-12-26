use itertools::Itertools;

struct Range {
    min: usize,
    max: usize,
}

impl Range {
    pub fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (min, max) = value
            .split_once("-")
            .expect("There was no bounds separator");

        Self {
            min: min.parse().expect("Left bound was not integer"),
            max: max.parse().expect("Right bound was not integer"),
        }
    }
}

fn part1(data: String) -> usize {
    data.lines()
        .into_iter()
        .map(parse_ranges)
        .filter(has_containment)
        .count()
}

fn parse_ranges(pair: &str) -> Vec<Range> {
    pair.split(",").map(Range::from).collect_vec()
}

fn has_containment(pair: &Vec<Range>) -> bool {
    pair[0].contains(&pair[1]) || pair[1].contains(&pair[0])
}

fn part2(data: String) -> usize {
    data.lines()
        .into_iter()
        .map(parse_ranges)
        .filter(has_overlap)
        .count()
}

fn has_overlap(pair: &Vec<Range>) -> bool {
    pair[0].overlaps(&pair[1])
}

aoc::main!(4);
