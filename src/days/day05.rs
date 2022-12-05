use std::collections::VecDeque;

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
        let stacks = Self::initial_stacks(start);

        format!("")
    }

    fn part2(&self) -> String {
        format!("")
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
                let cargo = lines[y]
                    .chars()
                    .nth(stack * 4 + 1).unwrap();
                if cargo.is_alphabetic() {
                    stacks[stack].push(cargo);
                }
            }
        }
        stacks
    }
}
