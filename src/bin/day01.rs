use std::mem;

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

fn part1(data: String) -> usize {
    let max_calories = data
        .split("\n\n")
        .map(count_calories)
        .max()
        .expect("No hay ningun grupo de elfos");

    max_calories
}

fn count_calories(elf: &str) -> usize {
    elf.lines()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .expect("All numbers should be valid")
        .into_iter()
        .sum()
}

fn part2(data: String) -> usize {
    let mut max_calories = Podium::new(3);

    data.split("\n\n")
        .map(count_calories)
        .for_each(|calories| max_calories.insert(calories));

    max_calories.sum()
}

aoc::main!(1);
