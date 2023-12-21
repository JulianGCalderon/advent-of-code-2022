use std::{error::Error, fs};

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn read_input(day: usize) -> BoxResult<String> {
    let path = format!("assets/input/input{:0>2}.txt", day);
    let input = fs::read_to_string(path)?;

    Ok(input)
}

#[macro_export]
macro_rules! main {
    ($day:expr) => {
        fn main() -> aoc::BoxResult<()> {
            let data = aoc::read_input($day)?;

            println!("Part1: {}", part1(data.clone())?);
            println!("Part2: {}", part2(data.clone())?);

            Ok(())
        }
    };
}
