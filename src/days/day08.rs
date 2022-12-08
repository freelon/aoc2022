use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day08 { input })
}

struct Day08 {
    input: String,
}

impl Day for Day08 {
    fn part1(&self) -> String {
        let (trees, h, w) = self.load_trees();

        trees
            .keys()
            .filter(|(tx, ty)| is_visible(&trees, *tx, *ty, h, w))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let (trees, h, w) = self.load_trees();

        trees
            .keys()
            .map(|(tx, ty)| scenic_score(&trees, *tx, *ty, h, w))
            .max()
            .unwrap()
            .to_string()
    } // 201600 too low
}

impl Day08 {
    fn load_trees(&self) -> (HashMap<(usize, usize), u32>, usize, usize) {
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
        (trees, h, w)
    }
}

fn is_visible(
    trees: &HashMap<(usize, usize), u32>,
    tx: usize,
    ty: usize,
    h: usize,
    w: usize,
) -> bool {
    let height = trees[&(tx, ty)];
    // invisible check
    let top = (0..ty).any(|y| trees[&(tx, y)] >= height);
    let bottom = (ty + 1..h).any(|y| trees[&(tx, y)] >= height);
    let left = (0..tx).any(|x| trees[&(x, ty)] >= height);
    let right = (tx + 1..w).any(|x| trees[&(x, ty)] >= height);
    // invert
    !(top && bottom && left && right)
}

fn scenic_score(
    trees: &HashMap<(usize, usize), u32>,
    tx: usize,
    ty: usize,
    h: usize,
    w: usize,
) -> usize {
    let height = trees[&(tx, ty)];
    // invisible check
    let top = (0..ty)
        .rev()
        .fold_while(0, |count, y| {
            if trees[&(tx, y)] >= height {
                Done(count + 1)
            } else {
                Continue(count + 1)
            }
        })
        .into_inner();
    let bottom = (ty + 1..h)
        .fold_while(0, |count, y| {
            if trees[&(tx, y)] >= height {
                Done(count + 1)
            } else {
                Continue(count + 1)
            }
        })
        .into_inner();
    let left = (0..tx)
        .rev()
        .fold_while(0, |count, x| {
            if trees[&(x, ty)] >= height {
                Done(count + 1)
            } else {
                Continue(count + 1)
            }
        })
        .into_inner();
    let right = (tx + 1..w)
        .fold_while(0, |count, x| {
            if trees[&(x, ty)] >= height {
                Done(count + 1)
            } else {
                Continue(count + 1)
            }
        })
        .into_inner();
    top * bottom * left * right
}

#[cfg(test)]
mod test {
    use crate::days::day08::{create, Day08, scenic_score};

    #[test]
    fn part2_a() {
        let (trees, h, w) = Day08 {
            input: EXAMPLE.to_string(),
        }
            .load_trees();
        assert_eq!(scenic_score(&trees, 2, 1, h, w), 4);
    }

    #[test]
    fn part2_b() {
        assert_eq!(create(EXAMPLE.to_string()).part2(), "8");
    }

    const EXAMPLE: &str = "30373
25512
65332
33549
35390
";
}
