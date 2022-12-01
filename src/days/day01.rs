use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day01 { input })
}

struct Day01 {
    input: String,
}

impl Day for Day01 {
    fn part1(&self) -> String {
        let snacks: i32 = self
            .input
            .split("\n\n")
            .map(|elf| elf.lines().map(|line| line.parse::<i32>().unwrap()).sum())
            .max()
            .unwrap();
        format!("{snacks}")
    }

    fn part2(&self) -> String {
        let mut snacks: Vec<i32> = self
            .input
            .split("\n\n")
            .map(|elf| elf.lines().map(|line| line.parse::<i32>().unwrap()).sum())
            .collect();
        snacks.sort();
        let x: i32 = snacks.iter().rev().take(3).sum();
        format!("{}", x)
    }
}
