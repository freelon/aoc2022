use std::ops::RangeInclusive;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day04 { input })
}

type R = RangeInclusive<u32>;

struct Day04 {
    input: String,
}

impl Day for Day04 {
    fn part1(&self) -> String {
        let fully_contained = self.count_filtered_pairs(fully_contains);
        format!("{fully_contained}")
    }

    fn part2(&self) -> String {
        let partial_contained = self.count_filtered_pairs(partially_contains);
        format!("{partial_contained}")
    }
}

impl Day04 {
    fn count_filtered_pairs(&self, filter: impl Fn(&R, &R) -> bool) -> usize {
        self.input
            .lines()
            .map(|line| {
                let (left, right) = line.split_once(',').unwrap();
                (to_range(left), to_range(right))
            })
            .filter(|(a, b)| filter(a, b) || filter(b, a))
            .count()
    }
}

fn to_range(s: &str) -> R {
    let (from, to) = s.split_once('-').unwrap();
    let (from, to): (u32, u32) = (from.parse().unwrap(), to.parse().unwrap());
    from..=to
}

fn fully_contains(a: &R, b: &R) -> bool {
    a.contains(b.start()) && a.contains(b.end())
}

fn partially_contains(a: &R, b: &R) -> bool {
    a.contains(b.start()) || a.contains(b.end())
}
