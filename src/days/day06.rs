use std::collections::{HashSet, VecDeque};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day06 { input })
}

struct Day06 {
    input: String,
}

impl Day for Day06 {
    fn part1(&self) -> String {
        let foo = self.consumed_until_different(4);
        format!("{}", foo)
    }

    fn part2(&self) -> String {
        let bar = self.consumed_until_different(14);
        format!("{}", bar)
    }
}

impl Day06 {
    fn consumed_until_different(&self, size: usize) -> usize {
        let slice: VecDeque<char> = self.input.chars().take(size).collect();
        let foo = self
            .input
            .chars()
            .skip(size)
            .fold_while((size, slice), |(consumed, mut slice), next| {
                let set: HashSet<_> = slice.iter().collect();
                if set.len() < size {
                    slice.pop_front();
                    slice.push_back(next);
                    Continue((consumed + 1, slice))
                } else {
                    Done((consumed, slice))
                }
            })
            .into_inner();
        foo.0
    }
}
