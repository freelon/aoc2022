use std::collections::HashSet;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day03 { input })
}

struct Day03 {
    input: String,
}

impl Day for Day03 {
    fn part1(&self) -> String {
        let sum_of_priorities: u32 = self.input.lines()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                let left: HashSet<char> = HashSet::from_iter(left.chars());
                let right: HashSet<char> = HashSet::from_iter(right.chars());
                let type_in_both = left.intersection(&right).next().unwrap();
                priority(type_in_both)
            })
            .sum();

        format!("{sum_of_priorities}")
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn priority(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        1 + *c as u32 - 'a' as u32
    } else {
        27 + *c as u32 - 'A' as u32
    }
}
