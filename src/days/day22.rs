use std::collections::HashMap;

use itertools::Itertools;

use crate::days::Day;
use crate::days::day22::Direction::{Down, Left, Right, Up};
use crate::days::day22::Tile::{Empty, Wall};

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day22::new(input))
}

type P = (i32, i32);

#[derive(Copy, Clone, PartialEq, Debug)]
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

    /// This function uses the move function of the example dice
    #[allow(dead_code)]
    fn move_dice(&self, state: &Day22, p: &P) -> (P, Direction) {
        let row_third = (p.1 - 1) / (state.height / 3);
        let col_quarter = (p.0 - 1) / (state.width / 4);
        let quarter = state.width / 4;
        let thirds = state.height / 3;

        match self {
            Right => {
                if p.0 < state.rows_mm[p.1 as usize].1 {
                    ((p.0 + 1, p.1), *self)
                } else {
                    match row_third {
                        0 => ((state.width, state.height - p.1 + 1), Left),
                        1 => {
                            let first_5_row = state.height / 3 * 2 + 1;
                            let from_first_5 = first_5_row - p.1;
                            ((p.0 + from_first_5, first_5_row), Down)
                        }
                        2 => ((state.width, state.height - p.1 + 1), Left),
                        _ => unreachable!("row_third too high"),
                    }
                }
            }
            Left => {
                if p.0 > state.rows_mm[p.1 as usize].0 {
                    ((p.0 - 1, p.1), *self)
                } else {
                    match row_third {
                        0 => {
                            let first_4_row = state.height / 3 + 1;
                            let from_first_4 = first_4_row - p.1;
                            ((p.0 + from_first_4, first_4_row), Down)
                        }
                        1 => {
                            let first_5_row = state.height / 3 * 2 + 1;
                            let from_first_5 = first_5_row - p.1;
                            ((state.width - (quarter - from_first_5), state.height), Up)
                        }
                        2 => {
                            let last_4_row = state.height / 3 * 2;
                            let from_last_4 = p.1 - last_4_row;
                            ((p.0 + from_last_4, last_4_row), Up)
                        }
                        _ => unreachable!("row_third too high"),
                    }
                }
            }
            Up => {
                if p.1 > state.cols_mm[p.0 as usize].0 {
                    ((p.0, p.1 - 1), *self)
                } else {
                    match col_quarter {
                        0 => ((3 * quarter - p.0, 1), Down),
                        1 => ((2 * quarter + 1, p.0 - quarter), Right),
                        2 => ((quarter - (3 * quarter - p.0), thirds + 1), Down),
                        3 => ((3 * quarter, thirds + (state.width - p.0) + 1), Left),
                        _ => unreachable!(),
                    }
                }
            }
            Down => {
                if p.1 < state.cols_mm[p.0 as usize].1 {
                    ((p.0, p.1 + 1), *self)
                } else {
                    match col_quarter {
                        0 => ((2 * quarter + 1 + (quarter - p.0), state.height), Up),
                        1 => (
                            (2 * quarter + 1, 2 * thirds + 1 + (2 * quarter - p.0)),
                            Right,
                        ),
                        2 => ((1 + (3 * quarter - p.0), 2 * thirds), Up),
                        3 => ((1, thirds + 1 + (state.height - p.0)), Left),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    fn move_dice2(&self, state: &Day22, p: &P) -> (P, Direction) {
        let row_quarter = (p.1 - 1) / (state.height / 4);
        let col_third = (p.0 - 1) / (state.width / 3);
        let quarter = state.height / 4;
        let thirds = state.width / 3;

        match self {
            Right => {
                if p.0 < state.rows_mm[p.1 as usize].1 {
                    ((p.0 + 1, p.1), *self)
                } else {
                    match row_quarter {
                        0 => ((2 * thirds, 3 * quarter + 1 - p.1), Left),
                        1 => ((2 * thirds + p.1 - quarter, quarter), Up),
                        2 => ((state.width, 1 + (3 * quarter - p.1)), Left),
                        3 => ((thirds + p.1 - 3 * quarter, 3 * quarter), Up),
                        _ => unreachable!("row_quarter too high"),
                    }
                }
            }
            Left => {
                if p.0 > state.rows_mm[p.1 as usize].0 {
                    ((p.0 - 1, p.1), *self)
                } else {
                    match row_quarter {
                        0 => ((1, 3 * quarter + 1 - p.1), Right),
                        1 => ((thirds - (2 * quarter - p.1), 2 * quarter + 1), Down),
                        2 => ((thirds + 1, 1 + (3 * quarter - p.1)), Right),
                        3 => ((p.1 - 3 * quarter + thirds, 1), Down),
                        _ => unreachable!("row_quarter too high"),
                    }
                }
            }
            Up => {
                if p.1 > state.cols_mm[p.0 as usize].0 {
                    ((p.0, p.1 - 1), *self)
                } else {
                    match col_third {
                        0 => ((thirds + 1, quarter + p.0), Right),
                        1 => ((1, 3 * quarter + (p.0 - thirds)), Right),
                        2 => ((p.0 - 2 * thirds, state.height), Up),
                        _ => unreachable!(),
                    }
                }
            }
            Down => {
                if p.1 < state.cols_mm[p.0 as usize].1 {
                    ((p.0, p.1 + 1), *self)
                } else {
                    match col_third {
                        0 => ((p.0 + 2 * thirds, 1), Down),
                        1 => ((thirds, p.0 - thirds + 3 * quarter), Left),
                        2 => ((2 * thirds, p.0 - 2 * thirds + quarter), Left),
                        _ => unreachable!(),
                    }
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
    width: i32,
    height: i32,
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

        Day22 {
            map,
            moves: moves.to_string(),
            rows_mm,
            cols_mm,
            width,
            height,
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
            let digits = moves
                .take_while_ref(|c| c.is_ascii_digit())
                .collect::<String>();
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
        let mut position = *self
            .map
            .keys()
            .filter(|(_, y)| y == &1)
            .min_by_key(|(x, _)| x)
            .unwrap();
        let mut direction = Right;
        let mut moves = self.moves.chars();
        loop {
            let digits = moves
                .take_while_ref(|c| c.is_ascii_digit())
                .collect::<String>();
            if !digits.is_empty() {
                let mut m: usize = digits.parse().unwrap();
                'moves: while m > 0 {
                    let (target_pos, new_direction) = direction.move_dice2(self, &position);
                    if let Some(Wall) = self.map.get(&target_pos) {
                        break 'moves;
                    }
                    position = target_pos;
                    direction = new_direction;
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
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day22::{Day22, Direction, P};
    use crate::days::day22::Direction::*;

    #[test]
    fn part1() {
        assert_eq!(Day22::new(EXAMPLE.to_string()).part1(), "6032");
    }

    #[test]
    fn part2() {
        // deactivated because it only works if day.part2() uses the move_dice function
        //assert_eq!(Day22::new(EXAMPLE.to_string()).part2(), "5031");
    }

    #[test]
    fn part2_teleport() {
        let statics = Day22::new(EXAMPLE.to_string());
        assert_eq!((Right.move_dice(&statics, &(12, 6))), ((15, 9), Down));
        assert_eq!((Down.move_dice(&statics, &(11, 12))), ((2, 8), Up));
    }

    #[test]
    fn part2_round_trip() {
        let statics = Day22::new(INPUT_SAMPLE.to_string());

        for (l, start, stop) in &[
            ("1", ((5, 1), Left), ((1, 12), Right)),
            ("2", ((5, 1), Up), ((1, 13), Right)),
            ("3", ((12, 1), Right), ((8, 12), Left)),
            ("4", ((12, 1), Up), ((4, 16), Up)),
            ("5", ((9, 4), Down), ((8, 5), Left)),
            ("6", ((5, 5), Left), ((1, 9), Down)),
            ("7", ((8, 5), Right), ((9, 4), Up)),
            ("8", ((1, 9), Left), ((5, 4), Right)),
            ("9", ((1, 9), Up), ((5, 5), Right)),
            ("10", ((8, 9), Right), ((12, 4), Left)),
            ("11", ((6, 12), Down), ((4, 14), Left)),
            ("12", ((1, 13), Left), ((5, 1), Down)),
            ("13", ((4, 15), Right), ((7, 12), Up)),
            ("14", ((4, 16), Down), ((12, 1), Down)),
        ] {
            let result = mov(&statics, *start);
            assert_eq!(result, *stop, "failed @ {l}");
        }
    }

    fn mov(statics: &Day22, start: (P, Direction)) -> (P, Direction) {
        let (p, d) = start;
        d.move_dice2(&statics, &p)
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

    const INPUT_SAMPLE: &str = "    ########
    ########
    ########
    ########
    ####
    ####
    ####
    ####
########
########
########
########
####
####
####
####

16";
}
