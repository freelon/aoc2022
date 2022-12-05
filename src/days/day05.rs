#![feature(slice_take)]

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(DayXX { input })
}

struct DayXX {
    input: String,
}

impl Day for DayXX {
    fn part1(&self) -> String {
        let (start, moves) = self.input.split_once("\n\n").unwrap();
        let mut stacks = Self::initial_stacks(start);

        for command in moves.lines() {
            let parts: Vec<&str> = command.split(" ").collect();
            let (times, from, to): (usize, usize, usize) = (
                parts[1].parse().unwrap(),
                parts[3].parse().unwrap(),
                parts[5].parse().unwrap(),
            );
            for _ in 0..times {
                let cargo = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(cargo);
            }
        }

        let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
        format!("{result}")
    }

    fn part2(&self) -> String {
        let (start, moves) = self.input.split_once("\n\n").unwrap();
        let mut stacks = Self::initial_stacks(start);

        for command in moves.lines() {
            let parts: Vec<&str> = command.split(" ").collect();
            let (times, from, to): (usize, usize, usize) = (
                parts[1].parse().unwrap(),
                parts[3].parse().unwrap(),
                parts[5].parse().unwrap(),
            );

            let from = stacks.get_mut(from - 1).unwrap();
            let mut temp = from.split_off(from.len() - times);
            stacks[to - 1].append(&mut temp);
        }

        let result: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
        format!("{result}")
    }
}

impl DayXX {
    fn initial_stacks(start: &str) -> Vec<Vec<char>> {
        let lines: Vec<&str> = start.lines().collect();
        let stacks = (lines[0].len() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stacks];

        let highest_stack = stacks.len() - 1; // last line is the stack number
        for y in (0..highest_stack).rev() {
            for stack in 0..stacks.len() {
                let cargo = lines[y].chars().nth(stack * 4 + 1).unwrap();
                if cargo.is_alphabetic() {
                    stacks[stack].push(cargo);
                }
            }
        }
        stacks
    }
}
