use aoc::BoxResult;

fn part1(data: String) -> BoxResult<usize> {
    let points: usize = data.lines().flat_map(points1).sum();

    Ok(points)
}

fn points1(line: &str) -> BoxResult<usize> {
    let regex = regex::Regex::new(r"(A|B|C) (X|Y|Z)")?;
    let capture = regex.captures(line).ok_or("Malformed input")?;

    let (_, [oponent, player]) = capture.extract();

    let oponent = Play::from_oponent(oponent.as_bytes()[0]);
    let strategy = Play::from_player(player.as_bytes()[0]);

    Ok(strategy.score() + strategy.battle(oponent).score())
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Play {
    Rock = 0,
    Paper,
    Scissors,
}

impl Play {
    pub fn from_oponent(mut oponent: u8) -> Self {
        oponent -= b'A';

        unsafe { std::mem::transmute(oponent) }
    }

    pub fn from_player(mut player: u8) -> Self {
        player -= b'X';

        unsafe { std::mem::transmute(player) }
    }

    pub fn score(self) -> usize {
        self as usize + 1
    }

    pub fn battle(self, other: Self) -> Outcome {
        let result = (self as u8 + 4 - other as u8) % 3;

        unsafe { std::mem::transmute(result) }
    }

    fn needed_for(oponent: Play, result: Outcome) -> Play {
        let strategy = (result as u8 + 2 + oponent as u8) % 3;

        unsafe { std::mem::transmute(strategy) }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn score(self) -> usize {
        self as usize * 3
    }

    pub fn from_result(mut result: u8) -> Self {
        result -= b'X';

        unsafe { std::mem::transmute(result) }
    }
}

fn part2(data: String) -> BoxResult<usize> {
    let points: usize = data.lines().flat_map(points2).sum();

    Ok(points)
}

fn points2(line: &str) -> BoxResult<usize> {
    let regex = regex::Regex::new(r"(A|B|C) (X|Y|Z)")?;
    let capture = regex.captures(line).ok_or("Malformed input")?;

    let (_, [oponent, result]) = capture.extract();

    let oponent = Play::from_oponent(oponent.as_bytes()[0]);
    let result = Outcome::from_result(result.as_bytes()[0]);
    let strategy = Play::needed_for(oponent, result);

    Ok(strategy.score() + result.score())
}

aoc::main!(2);
