use itertools::Itertools;

struct Range(usize, usize);

impl Range {
    pub fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (min, max) = value
            .splitn(2, "-")
            .flat_map(str::parse::<usize>)
            .collect_tuple()
            .expect("Range was malformed");

        Self(min, max)
    }
}

fn part1(data: String) -> usize {
    data.lines()
        .into_iter()
        .map(parse_ranges)
        .filter(has_containment)
        .count()
}

fn parse_ranges(pair: &str) -> (Range, Range) {
    pair.split(",")
        .map(Range::from)
        .collect_tuple()
        .expect("Input was malformed")
}

fn has_containment(pair: &(Range, Range)) -> bool {
    pair.0.contains(&pair.1) || pair.1.contains(&pair.0)
}

fn part2(data: String) -> usize {
    data.lines()
        .into_iter()
        .map(parse_ranges)
        .filter(has_overlap)
        .count()
}

fn has_overlap(pair: &(Range, Range)) -> bool {
    pair.0.overlaps(&pair.1)
}

aoc::main!(4);
