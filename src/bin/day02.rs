use aoc::BoxResult;

fn part1(data: String) -> BoxResult<usize> {
    let points: usize = data.lines().flat_map(points).sum();

    Ok(points)
}

fn points(line: &str) -> BoxResult<usize> {
    let _ = line;
    Ok(1)
}

fn part2(data: String) -> BoxResult<usize> {
    let _ = data;
    Ok(0)
}

aoc::main!(2);
