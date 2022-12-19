use std::cmp::max;
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
        let points = self.parse();

        points
            .iter()
            .map(|&(x, y, z)| {
                Self::neighbors(x, y, z)
                    .into_iter()
                    .filter(|p| !points.contains(p))
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let points = self.parse();

        let (x_max, y_max, z_max) = points
            .iter()
            .fold((-1, -1, -1), |(x_max, y_max, z_max), p| {
                (max(x_max, p.0), max(y_max, p.1), max(z_max, p.2))
            });

        let mut visited: HashSet<P> = HashSet::new();
        let mut stack = vec![(-1, -1, -1)];
        let mut neighbor_is_lava = 0;
        while let Some(p) = stack.pop() {
            if visited.contains(&p) {
                continue;
            }
            let (x, y, z) = p;
            let n = Self::neighbors(x, y, z);

            let non_lava_neighbors = n
                .into_iter()
                .filter(|it| !points.contains(it))
                .collect_vec();
            neighbor_is_lava += 6 - non_lava_neighbors.len();
            for (a, b, c) in non_lava_neighbors {
                if a >= -1
                    && a <= x_max + 1
                    && b >= -1
                    && b <= y_max + 1
                    && c >= -1
                    && c <= z_max + 1
                {
                    stack.push((a, b, c));
                }
            }

            visited.insert(p);
        }
        neighbor_is_lava.to_string()
    }
}

impl Day18 {
    fn parse(&self) -> HashSet<P> {
        let points: HashSet<P> = self
            .input
            .lines()
            .map(|line| line.splitn(3, ',').collect_tuple().unwrap())
            .map(|(x, y, z)| (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()))
            .collect();
        points
    }

    fn neighbors(x: i32, y: i32, z: i32) -> [(i32, i32, i32); 6] {
        [
            (x + 1, y, z),
            (x, y + 1, z),
            (x, y, z + 1),
            (x - 1, y, z),
            (x, y - 1, z),
            (x, y, z - 1),
        ]
    }
}
