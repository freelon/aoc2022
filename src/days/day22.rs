use std::collections::HashMap;

use itertools::Itertools;

use crate::days::Day;
use crate::days::day22::Direction::{Down, Left, Right, Up};
use crate::days::day22::Tile::{Empty, Wall};

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day22::new(input))
}

type P = (i32, i32);

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Right => Up,
            Left => Down,
            Up => Left,
            Down => Right,
        }
    }

    fn right(&self) -> Self {
        match self {
            Right => Down,
            Left => Up,
            Up => Right,
            Down => Left,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Right => 0,
            Left => 2,
            Up => 3,
            Down => 1,
        }
    }

    fn moved(&self, state: &Day22, p: &P) -> P {
        match self {
            Right => {
                if p.0 < state.rows_mm[p.1 as usize].1 {
                    (p.0 + 1, p.1)
                } else {
                    (state.rows_mm[p.1 as usize].0, p.1)
                }
            }
            Left => {
                if p.0 > state.rows_mm[p.1 as usize].0 {
                    (p.0 - 1, p.1)
                } else {
                    (state.rows_mm[p.1 as usize].1, p.1)
                }
            }
            Up => {
                if p.1 > state.cols_mm[p.0 as usize].0 {
                    (p.0, p.1 - 1)
                } else {
                    (p.0, state.cols_mm[p.0 as usize].1)
                }
            }
            Down => {
                if p.1 < state.cols_mm[p.0 as usize].1 {
                    (p.0, p.1 + 1)
                } else {
                    (p.0, state.cols_mm[p.0 as usize].0)
                }
            }
        }
    }
}

enum Tile {
    Empty,
    Wall,
}

struct Day22 {
    map: HashMap<P, Tile>,
    moves: String,
    rows_mm: Vec<(i32, i32)>,
    cols_mm: Vec<(i32, i32)>,
}

impl Day22 {
    fn new(input: String) -> Day22 {
        let (tiles, moves) = input.split_once("\n\n").unwrap();
        let map: HashMap<P, Tile> = tiles
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .map(move |(x, c)| {
                        (
                            (x as i32 + 1, y as i32 + 1),
                            match c {
                                '#' => Wall,
                                '.' => Empty,
                                _ => unreachable!("unknown input char"),
                            },
                        )
                    })
            })
            .collect();
        let width = map.keys().max_by_key(|(x, _)| x).unwrap().0;
        let height = map.keys().max_by_key(|(_, y)| y).unwrap().1;
        let rows_mm = (0..=height)
            .map(|y| {
                (
                    map.keys()
                        .filter(|(_, ry)| *ry == y)
                        .min_by_key(|(x, _)| x)
                        .unwrap_or(&(0, 0))
                        .0,
                    map.keys()
                        .filter(|(_, ry)| *ry == y)
                        .max_by_key(|(x, _)| x)
                        .unwrap_or(&(0, 0))
                        .0,
                )
            })
            .collect();
        let cols_mm = (0..=width)
            .map(|x| {
                (
                    map.keys()
                        .filter(|(rx, _)| *rx == x)
                        .min_by_key(|(_, y)| y)
                        .unwrap_or(&(0, 0))
                        .1,
                    map.keys()
                        .filter(|(rx, _)| *rx == x)
                        .max_by_key(|(_, y)| y)
                        .unwrap_or(&(0, 0))
                        .1,
                )
            })
            .collect();

        println!("{:?}", rows_mm);
        println!("{:?}", cols_mm);
        Day22 {
            map,
            moves: moves.to_string(),
            rows_mm,
            cols_mm,
        }
    }
}

impl Day for Day22 {
    fn part1(&self) -> String {
        let mut position = *self
            .map
            .keys()
            .filter(|(_, y)| y == &1)
            .min_by_key(|(x, _)| x)
            .unwrap();
        let mut direction = Right;
        let mut moves = self.moves.chars();
        loop {
            let digits = moves.take_while_ref(|c| c.is_ascii_digit()).collect::<String>();
            if !digits.is_empty() {
                let mut m: usize = digits.parse().unwrap();
                'moves: while m > 0 {
                    let target_pos: P = direction.moved(self, &position);
                    if let Some(Wall) = self.map.get(&target_pos) {
                        break 'moves;
                    }
                    position = target_pos;
                    m -= 1;
                }
            }
            if let Some(c) = moves.next() {
                direction = match c {
                    'L' => direction.left(),
                    'R' => direction.right(),
                    _ => unreachable!("invalid turn"),
                };
            } else {
                break;
            }
        }
        (1000 * position.1 + 4 * position.0 + direction.value()).to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day22::Day22;

    #[test]
    fn part1() {
        assert_eq!(Day22::new(EXAMPLE.to_string()).part1(), "6032");
    }

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
}
