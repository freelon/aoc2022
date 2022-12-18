use std::collections::HashSet;

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day18 { input })
}

struct Day18 {
    input: String,
}

type P = (i32, i32, i32);

impl Day for Day18 {
    fn part1(&self) -> String {
        let points: HashSet<P> = self
            .input
            .lines()
            .map(|line| line.splitn(3, ',').collect_tuple().unwrap())
            .map(|(x, y, z)| (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()))
            .collect();

        points
            .iter()
            .map(|&(x, y, z)| {
                let n = [
                    (x + 1, y, z),
                    (x, y + 1, z),
                    (x, y, z + 1),
                    (x - 1, y, z),
                    (x, y - 1, z),
                    (x, y, z - 1),
                ];
                n.into_iter().filter(|p| !points.contains(p)).count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}
