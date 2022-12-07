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
        let result = self.consumed_until_different(4);
        result.to_string()
    }

    fn part2(&self) -> String {
        let result = self.consumed_until_different(14);
        result.to_string()
    }
}

impl Day06 {
    fn consumed_until_different(&self, size: usize) -> usize {
        let vec: Vec<_> = self.input.chars().collect();
        vec.windows(size)
            .fold_while(size, |consumed, next| {
                if unique_chars(next) {
                    return Done(consumed);
                }
                Continue(consumed + 1)
            })
            .into_inner()
    }
}

fn unique_chars(chars: &[char]) -> bool {
    for i in 1..chars.len() {
        if chars[0..i].contains(&chars[i]) {
            return false;
        }
    }
    true
}
