struct Range {
    min: usize,
    max: usize,
}

impl Range {
    pub fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &&Range) -> bool {
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

fn parse_ranges(pair: &str) -> (Range, Range) {
    let (left, right) = pair.split_once(",").expect("There was no range separator");

    (left.into(), right.into())
}

fn has_containment((left, right): &(Range, Range)) -> bool {
    left.contains(&right) || right.contains(&left)
}

fn part2(data: String) -> usize {
    data.lines()
        .into_iter()
        .map(parse_ranges)
        .filter(has_overlap)
        .count()
}

fn has_overlap((left, right): &(Range, Range)) -> bool {
    left.overlaps(&right)
}

aoc::main!(4);
