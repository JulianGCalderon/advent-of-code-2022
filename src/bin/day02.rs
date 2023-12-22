use aoc::BoxResult;

fn part1(data: String) -> BoxResult<usize> {
    let points: usize = data.lines().flat_map(points).sum();

    Ok(points)
}

fn points(line: &str) -> BoxResult<usize> {
    let regex = regex::Regex::new(r"(A|B|C) (X|Y|Z)")?;
    let capture = regex.captures(line).ok_or("Malformed input")?;

    let (_, [oponent, strategy]) = capture.extract();

    let oponent = Play::from_oponent(oponent.as_bytes()[0]);
    let strategy = Play::from_player(strategy.as_bytes()[0]);

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
}

#[allow(dead_code)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn score(self) -> usize {
        self as usize * 3
    }
}

fn part2(data: String) -> BoxResult<usize> {
    let _ = data;
    Ok(0)
}

aoc::main!(2);
