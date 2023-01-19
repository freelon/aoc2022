use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day24 { input })
}

struct Day24 {
    input: String,
}

type P = (i32, i32);

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    e: P,
    blizzards: Vec<(P, char)>,
    x_wall_left: i32,
    x_wall_right: i32,
    y_wall_top: i32,
    y_wall_bottom: i32,
}

impl State {
    fn advance(&self) -> Self {
        let new_blizzards = self
            .blizzards
            .iter()
            .map(|&((x, y), c)| match c {
                '<' => {
                    if x == self.x_wall_left + 1 {
                        ((self.x_wall_right - 1, y), c)
                    } else {
                        ((x - 1, y), c)
                    }
                }
                '>' => {
                    if x == self.x_wall_right - 1 {
                        ((self.x_wall_left + 1, y), c)
                    } else {
                        ((x + 1, y), c)
                    }
                }
                'v' => {
                    if y == self.y_wall_bottom - 1 {
                        ((x, self.y_wall_top + 1), c)
                    } else {
                        ((x, y + 1), c)
                    }
                }
                '^' => {
                    if y == self.y_wall_top + 1 {
                        ((x, self.y_wall_bottom - 1), c)
                    } else {
                        ((x, y - 1), c)
                    }
                }
                _ => unreachable!("bitch!"),
            })
            .collect_vec();

        State {
            blizzards: new_blizzards,
            ..*self
        }
    }

    fn valid(&self) -> bool {
        let e = self.e;
        if e.0 <= self.x_wall_left
            || e.0 >= self.x_wall_right
            || (e.1 <= self.y_wall_top && !(e.0 == 1))
            || (e.1 >= self.y_wall_bottom && !(e.0 == self.x_wall_right - 1))
        {
            return false;
        }

        self.blizzards
            .iter()
            .filter(|(p, _c)| *p == e)
            .next()
            .is_none()
    }
}

impl Day for Day24 {
    fn part1(&self) -> String {
        let start_blizzards = self
            .input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| ['<', '>', 'v', '^'].contains(c))
                    .map(move |(x, c)| ((x as i32, y as i32), c))
            })
            .collect_vec();
        let x_wall_left = 0;
        let x_wall_right = (self.input.lines().next().unwrap().len() - 1) as i32;
        let y_wall_top = 0;
        let y_wall_bottom = (self.input.lines().count() - 1) as i32;

        let goal = (x_wall_right - 1, y_wall_bottom);

        let start = State {
            e: (1, 0),
            blizzards: start_blizzards,
            x_wall_left,
            x_wall_right,
            y_wall_top,
            y_wall_bottom,
        };

        let mut open: VecDeque<(State, usize)> = VecDeque::new();
        let mut visited: HashMap<State, usize> = HashMap::new();

        open.push_back((start, 0));

        while let Some((state, time)) = open.pop_front() {
            //println!("inspecting position {:?} at time {time}", state.e);
            if visited.contains_key(&state) {
                continue;
            }

            if state.e == goal {
                return format!("{time}");
            }

            for delta in &[(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut next = state.advance();
                let e = add(delta, &next.e);
                next.e = e;

                if next.valid() {
                    open.push_back((next, time + 1));
                }
            }

            visited.insert(state, time);
        }

        unreachable!("there was no solution :o")
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn add(p0: &P, p1: &P) -> P {
    (p0.0 + p1.0, p0.1 + p1.1)
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day24::Day24;

    #[test]
    fn example1_part1() {
        let day = Day24 {
            input: INPUT.to_string(),
        };
        assert_eq!(day.part1(), "18");
    }

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
}
