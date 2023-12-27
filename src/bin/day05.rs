use itertools::Itertools;
use regex::Regex;

type Ship = Vec<Stack>;
type Stack = Vec<char>;

struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Should be valid");

        let (amount, from, to) = regex
            .captures(line)
            .expect("Instruction was malformed")
            .extract::<3>()
            .1
            .into_iter()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .expect("There should be 3 digits")
            .into_iter()
            .collect_tuple()
            .expect("Should be valid digits");

        Self {
            amount,
            from: from - 1,
            to: to - 1,
        }
    }
}

fn part1(data: String) -> String {
    let (ship, instructions) = data.split_once("\n\n").expect("There was no empty line");

    let mut ship = parse_ship(ship);

    instructions
        .lines()
        .map(Instruction::from)
        .for_each(|instruction| execute(&mut ship, instruction));

    ship.iter()
        .map(|stack| stack.last())
        .collect::<Option<_>>()
        .expect("No stack should be empty")
}

fn parse_ship(layout: &str) -> Ship {
    let mut ship = Vec::new();

    let mut lines = layout.lines();
    lines.next_back(); // Ignores stack number line

    for line in lines {
        let chests = line.chars().skip(1).step_by(4).enumerate();

        for (i, chest) in chests {
            if i == ship.len() {
                ship.push(Vec::new());
            }

            if chest != ' ' {
                ship[i].push(chest);
            }
        }
    }

    for stack in &mut ship {
        stack.reverse();
    }

    ship
}

fn execute(ship: &mut Ship, instruction: Instruction) {
    for _ in 0..instruction.amount {
        let chest = ship[instruction.from]
            .pop()
            .expect("Stack should not be empty");

        ship[instruction.to].push(chest);
    }
}

fn part2(data: String) -> String {
    let (ship, instructions) = data.split_once("\n\n").expect("There was no empty line");

    let mut ship = parse_ship(ship);

    instructions
        .lines()
        .map(Instruction::from)
        .for_each(|instruction| execute2(&mut ship, instruction));

    ship.iter()
        .map(|stack| stack.last())
        .collect::<Option<_>>()
        .expect("No stack should be empty")
}

fn execute2(ship: &mut Ship, instruction: Instruction) {
    let origin = &mut ship[instruction.from];

    let mut moved = origin
        .drain((origin.len() - instruction.amount)..)
        .collect_vec();

    ship[instruction.to].append(&mut moved);
}

aoc::main!(5);
