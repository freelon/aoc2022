use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day03 { input })
}

struct Day03 {
    input: String,
}

impl Day for Day03 {
    fn part1(&self) -> String {
        let sum_of_priorities: u32 = self
            .input
            .lines()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                let type_in_both = left.chars().find(|c| right.contains(*c)).unwrap();
                priority(type_in_both)
            })
            .sum();

        format!("{sum_of_priorities}")
    }

    fn part2(&self) -> String {
        let sum_of_triplet_priorities: u32 = self
            .input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|x| {
                let (a, b, c) = x.into_iter().next_tuple().unwrap();
                let in_all = a
                    .chars()
                    .filter(|x| b.contains(*x))
                    .find(|x| c.contains(*x))
                    .unwrap();
                priority(in_all)
            })
            .sum();
        format!("{sum_of_triplet_priorities}")
    }
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        1 + c as u32 - 'a' as u32
    } else {
        27 + c as u32 - 'A' as u32
    }
}
