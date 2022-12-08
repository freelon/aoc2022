use std::collections::HashMap;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day08 { input })
}

struct Day08 {
    input: String,
}

impl Day for Day08 {
    fn part1(&self) -> String {
        let trees: HashMap<(usize, usize), u32> = self
            .input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x, y), c.to_digit(10).unwrap()))
            })
            .collect();

        let h = self.input.lines().count();
        let w = self.input.lines().next().unwrap().len();

        trees
            .keys()
            .filter(|(tx, ty)| is_visible(&trees, *tx, *ty, h, w))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn is_visible(trees: &HashMap<(usize, usize), u32>, tx: usize, ty: usize, h: usize, w: usize) -> bool {
    let height = trees[&(tx, ty)];
    // invisible check
    let top = (0..ty).any(|y| trees[&(tx, y)] >= height);
    let bottom = (ty + 1..h).any(|y| trees[&(tx, y)] >= height);
    let left = (0..tx).any(|x| trees[&(x, ty)] >= height);
    let right = (tx + 1..w).any(|x| trees[&(x, ty)] >= height);
    !(top && bottom && left && right)
}
