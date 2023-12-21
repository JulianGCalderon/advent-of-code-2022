use std::{error::Error, mem};

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn part1(data: String) -> BoxResult<usize> {
    let max_calories = data
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<usize>).sum())
        .max()
        .ok_or("No hay ningun grupo de elfos")?;

    Ok(max_calories)
}

fn part2(data: String) -> BoxResult<usize> {
    let mut max_calories = Podium::new(3);

    data.split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<usize>).sum())
        .for_each(|calories| max_calories.insert(calories));

    Ok(max_calories.sum())
}

struct Podium(Vec<usize>);

impl Podium {
    pub fn new(size: usize) -> Self {
        Podium(Vec::with_capacity(size))
    }

    pub fn insert(&mut self, mut n: usize) {
        for i in 0..self.0.len() {
            if self.0[i] < n {
                mem::swap(&mut self.0[i], &mut n)
            }
        }

        if self.0.len() < 3 {
            self.0.push(n);
        }
    }

    pub fn sum(&self) -> usize {
        self.0.iter().sum()
    }
}

aoc::main!(1);
