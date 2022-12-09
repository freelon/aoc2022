use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub};

use derive_more::From;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day09 { input })
}

struct Day09 {
    input: String,
}

impl Day for Day09 {
    fn part1(&self) -> String {
        self.run(2).visited.len().to_string()
    }

    fn part2(&self) -> String {
        self.run(10).visited.len().to_string()
    }
}

impl Day09 {
    fn run(&self, tail_length: usize) -> State {
        let initial_state = State::new(tail_length);
        self.input
            .lines()
            .flat_map(|line| {
                let (dir, count) = line.split_once(' ').unwrap();
                vec![dir.chars().next().unwrap()].repeat(count.parse::<usize>().unwrap())
            })
            .fold(initial_state, |mut state, c| {
                let move_head = match c {
                    'U' => (0, 1).into(),
                    'D' => (0, -1).into(),
                    'L' => (-1, 0).into(),
                    'R' => (1, 0).into(),
                    _ => panic!("unknown movement {c}"),
                };

                state.knots[0] += move_head;

                for j in 1..state.knots.len() {
                    let move_tail = tail_move_direction(state.knots[j - 1], state.knots[j]);
                    state.knots[j] += move_tail;
                }
                state.visited.insert(*state.knots.last().unwrap());

                state
            })
    }
}

#[derive(Default)]
struct State {
    knots: Vec<P>,
    visited: HashSet<P>,
}

impl State {
    fn new(tail_length: usize) -> Self {
        let mut result = Self {
            knots: vec![(0, 0).into(); tail_length],
            visited: HashSet::default(),
        };
        result.visited.insert((0, 0).into());
        result
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug, From)]
struct P {
    x: i32,
    y: i32,
}

impl P {
    fn signum(&self) -> Self {
        (self.x.signum(), self.y.signum()).into()
    }
}

impl Add for P {
    type Output = P;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl AddAssign for P {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for P {
    type Output = P;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

fn tail_move_direction(h: P, t: P) -> P {
    let direction = h - t;

    if direction.x.abs() > 1 || direction.y.abs() > 1 {
        direction.signum()
    } else {
        (0, 0).into()
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day09::Day09;

    #[test]
    fn part1() {
        let day = Day09 {
            input: EXAMPLE.to_string(),
        };
        assert_eq!(day.part1(), "13");
    }

    #[test]
    fn part2() {
        let day = Day09 {
            input: EXAMPLE.to_string(),
        };
        assert_eq!(day.part2(), "1");
    }

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
}
