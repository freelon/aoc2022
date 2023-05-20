use std::collections::HashMap;
use std::collections::VecDeque;

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day17 { input })
}

struct Day17 {
    input: String,
}

type State = (usize, Vec<Row>, Vec<char>);

impl Day17 {
    fn run(&self, number_of_stones: usize) -> String {
        let mut next_stones = STONES
            .iter()
            .enumerate()
            .map(|(i, stone_str)| (i, to_stone_matrix(stone_str)))
            .cycle();
        let mut next_wind = self.input.trim().chars().cycle();
        let mut rows: VecDeque<Row> = VecDeque::with_capacity(100);
        let mut rows_offset = 0;
        rows.push_back(['#', '#', '#', '#', '#', '#', '#']);

        let mut run_number = 1;
        let mut states: HashMap<State, (usize, usize)> = HashMap::new();
        while run_number <= number_of_stones {
            let (stone_number, stone) = next_stones.next().unwrap();
            //         println!("new stone: {stone:?}");
            let state = (
                stone_number,
                rows.iter().rev().take(30).copied().collect_vec(),
                next_wind.clone().take(self.input.len()).collect_vec(),
            );
            run_number += 1;
            if let Some((old_round, highest)) = states.get(&state) {
                let height_diff = current_highest(&rows, rows_offset) - highest;
                let run_diff = run_number - old_round;
                let remaining_rounds = number_of_stones - run_number;
                let cycles_to_skip = remaining_rounds / run_diff;
                run_number += run_diff * cycles_to_skip;
                rows_offset += height_diff * cycles_to_skip;
                states.clear();
                // println!("found duplicate in round {run_number} in round {old_round}. Hight increased from {highest} to {}", current_highest(&rows, rows_offset));
            } else {
                states.insert(state, (run_number, current_highest(&rows, rows_offset)));
            }
            let mut stop = false;
            let mut bottom_x = 2;
            let mut bottom_y = current_highest(&rows, rows_offset) + 4; // we need 3 empty rows in between
            while rows.len() + rows_offset < bottom_y + stone.len() {
                rows.push_back([' ', ' ', ' ', ' ', ' ', ' ', ' '])
            }
            while rows.len() > 100 {
                rows.pop_front();
                rows_offset += 1;
            }

            while !stop {
                let jet = next_wind.next().unwrap();
                //             //             println!("new round, new jet: {jet}");
                //             //             println!("bottom left is now at {bottom_x},{bottom_y}");
                match jet {
                    '<' => {
                        if bottom_x > 0
                            && no_overlaps(&rows, rows_offset, bottom_x - 1, bottom_y, &stone)
                        {
                            bottom_x -= 1;
                        }
                    }
                    '>' => {
                        if bottom_x + stone[0].len() < 7
                            && no_overlaps(&rows, rows_offset, bottom_x + 1, bottom_y, &stone)
                        {
                            bottom_x += 1;
                        }
                    }
                    _ => {}
                }

                //             //             println!("checking falling down");
                if no_overlaps(&rows, rows_offset, bottom_x, bottom_y - 1, &stone) {
                    bottom_y -= 1;
                } else {
                    for x in 0..stone[0].len() {
                        for y in 0..stone.len() {
                            if stone[y][x] == '#' {
                                rows[bottom_y - rows_offset + y][bottom_x + x] = '#';
                            }
                        }
                    }
                    stop = true;
                }
            }
            print_rows(&rows, rows_offset);
        }

        format!("{}", current_highest(&rows, rows_offset))
    }
}

type Row = [char; 7];

impl Day for Day17 {
    fn part1(&self) -> String {
        self.run(2022)
    }

    fn part2(&self) -> String {
        self.run(1000000000000)
    }
}

fn to_stone_matrix(stone_str: &str) -> Vec<Vec<char>> {
    stone_str
        .lines()
        .rev()
        .map(|line| line.chars().collect())
        .collect()
}

fn no_overlaps(
    rows: &VecDeque<Row>,
    rows_offset: usize,
    bottom_left: usize,
    bottom_y: usize,
    stone: &Vec<Vec<char>>,
) -> bool {
    // // println!("no_overlaps?");
    // // println!("rock bottom left: {bottom_left},{bottom_y}");
    !(0..stone[0].len())
        .cartesian_product(0..stone.len())
        .any(|(x, y)| {
            //         //         println!("{x},{y} - {bottom_left},{bottom_y}");
            rows[bottom_y + y - rows_offset][bottom_left + x] == '#' && stone[y][x] == '#'
        })
}

fn current_highest(rows: &VecDeque<Row>, rows_offset: usize) -> usize {
    rows.iter()
        .enumerate()
        .rev()
        .find(|(_index, row)| row.iter().any(|c| *c == '#'))
        .map(|(index, _)| index)
        .unwrap()
        + rows_offset
}

fn print_rows(_rows: &VecDeque<Row>, _rows_offset: usize) {
    // rows.iter().rev().for_each(|line| {
    // //     println!("|{}|", line.iter().map(|c| *c).collect::<String>());
    // })
}

const STONES: &[&str] = &[
    "####
",
    ".#.
###
.#.
",
    "..#
..#
###
",
    "#
#
#
#
",
    "##
##",
];

#[cfg(test)]
mod test {
    use crate::days::Day;

    use super::Day17;

    #[test]
    fn part1() {
        assert_eq!(
            Day17 {
                input: ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()
            }
            .part1(),
            "3068"
        );
    }
    #[test]
    fn part2() {
        assert_eq!(
            Day17 {
                input: ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()
            }
            .part2(),
            "1514285714288"
        );
    }
}
