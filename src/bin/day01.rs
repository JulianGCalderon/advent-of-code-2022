use std::{
    cmp::max,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    mem,
};

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn part1(data: BufReader<File>) -> BoxResult<usize> {
    let mut max_calories = 0;
    let mut cur_calories = 0;

    for line in data.lines() {
        let line = line?;

        if line.is_empty() {
            max_calories = max(max_calories, cur_calories);
            cur_calories = 0;
        } else {
            cur_calories += line.parse::<usize>()?;
        }
    }
    max_calories = max(max_calories, cur_calories);

    Ok(max_calories)
}

pub fn insert(podium: &mut [usize], mut n: usize) {
    for i in 0..podium.len() {
        if podium[i] < n {
            mem::swap(&mut podium[i], &mut n)
        }
    }
}

fn part2(data: BufReader<File>) -> BoxResult<usize> {
    let mut max_calories = vec![0; 3];
    let mut cur_calories = 0;

    for line in data.lines() {
        let line = line?;

        if line.is_empty() {
            insert(&mut max_calories, cur_calories);
            cur_calories = 0;
        } else {
            cur_calories += line.parse::<usize>()?;
        }
    }

    insert(&mut max_calories, cur_calories);

    Ok(max_calories.iter().sum())
}

fn main() -> BoxResult<()> {
    let data = BufReader::new(File::open("assets/input/day01.txt")?);
    println!("Part1: {}", part1(data)?);

    let data = BufReader::new(File::open("assets/input/day01.txt")?);
    println!("Part2: {:?}", part2(data)?);

    Ok(())
}
