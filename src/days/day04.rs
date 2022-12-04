use std::ops::RangeInclusive;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day04 { input })
}

struct Day04 {
    input: String,
}

impl Day for Day04 {
    fn part1(&self) -> String {
        let fully_contained = self
            .input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(",").unwrap();
                (to_range(left), to_range(right))
            })
            .filter(|(a, b)| fully_contains(a, b) || fully_contains(b, a))
            .count();
        format!("{fully_contained}")
    }

    fn part2(&self) -> String {
        let partial_contained = self
            .input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(",").unwrap();
                (to_range(left), to_range(right))
            })
            .filter(|(a, b)| partially_contains(a, b) || partially_contains(b, a))
            .count();
        format!("{partial_contained}")
    }
}

fn to_range(s: &str) -> RangeInclusive<u32> {
    let (from, to) = s.split_once("-").unwrap();
    let (from, to): (u32, u32) = (from.parse().unwrap(), to.parse().unwrap());
    from..=to
}

fn fully_contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.contains(b.start()) && a.contains(b.end())
}

fn partially_contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.contains(b.start()) || a.contains(b.end())
}