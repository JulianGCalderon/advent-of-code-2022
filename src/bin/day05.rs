use itertools::Itertools;
use regex::Regex;

type Ship = Vec<Stack>;
type Stack = Vec<char>;
type Instruction = (usize, usize, usize);

fn part1(data: String) -> String {
    let (ship, instructions) = data.split_once("\n\n").expect("There was no empty line");

    let mut ship = parse_ship(ship);

    instructions
        .lines()
        .map(parse_instruction)
        .for_each(|instruction| execute(&mut ship, instruction));

    ship.iter().flat_map(|stack| stack.last()).collect()
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

fn parse_instruction(line: &str) -> Instruction {
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Should be valid");

    let (_, instruction) = regex
        .captures(line)
        .expect("Instruction was malformed")
        .extract::<3>();

    instruction
        .into_iter()
        .flat_map(str::parse)
        .collect_tuple()
        .expect("Should be valid digits")
}

fn execute(ship: &mut Ship, instruction: Instruction) {
    for _ in 0..instruction.0 {
        let chest = ship[instruction.1 - 1]
            .pop()
            .expect("Stack should not be empty");

        ship[instruction.2 - 1].push(chest);
    }
}

fn part2(data: String) -> String {
    let (ship, instructions) = data.split_once("\n\n").expect("There was no empty line");

    let mut ship = parse_ship(ship);

    instructions
        .lines()
        .map(parse_instruction)
        .for_each(|instruction| execute2(&mut ship, instruction));

    ship.iter().flat_map(|stack| stack.last()).collect()
}

fn execute2(ship: &mut Ship, instruction: Instruction) {
    let origin = &mut ship[instruction.1 - 1];

    let mut moved = origin.drain((origin.len() - instruction.0)..).collect_vec();

    ship[instruction.2 - 1].append(&mut moved);
}

aoc::main!(5);
