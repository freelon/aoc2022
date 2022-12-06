use std::collections::HashSet;

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
        let foo = self.input.chars().tuple_windows::<(char, char, char, char)>().enumerate().find(|(index, chars)| {
            let mut set = HashSet::new();
            set.insert(chars.0);
            set.insert(chars.1);
            set.insert(chars.2);
            set.insert(chars.3);
            set.len() == 4
        }).unwrap();
        format!("{}", foo.0 + 4)
    }

    fn part2(&self) -> String {
        format!("")
    }
}
