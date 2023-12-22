use std::{fs, io};

pub fn read_input(day: usize) -> io::Result<String> {
    let path = format!("input/input{:0>2}.txt", day);

    fs::read_to_string(path)
}

#[macro_export]
macro_rules! main {
    ($day:expr) => {
        fn main() {
            let data = match aoc::read_input($day) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("Could not find input file for day {}", $day);
                    return;
                }
            };

            println!("Part 1: {}", part1(data.clone()));
            println!("Part 2: {}", part2(data));
        }
    };
}
