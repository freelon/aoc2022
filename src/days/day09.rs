use std::collections::HashSet;

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
                    'U' => P::new(0, 1),
                    'D' => P::new(0, -1),
                    'L' => P::new(-1, 0),
                    'R' => P::new(1, 0),
                    _ => panic!("unknown movement {c}"),
                };

                state.knots[0] = state.knots[0].add(move_head);

                for j in 1..state.knots.len() {
                    let move_tail = tail_move_direction(state.knots[j - 1], state.knots[j]);
                    state.knots[j] = state.knots[j].add(move_tail);
                }
                state.visited.insert(state.knots[state.knots.len() - 1]);

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
            knots: vec![P::new(0, 0); tail_length],
            visited: HashSet::default(),
        };
        result.visited.insert(P::new(0, 0));
        result
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct P {
    x: i32,
    y: i32,
}

impl P {
    fn new(x: i32, y: i32) -> Self {
        P { x, y }
    }

    fn add(&self, other: P) -> P {
        Self::new(self.x + other.x, self.y + other.y)
    }

    fn sub(&self, other: P) -> P {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

fn tail_move_direction(h: P, t: P) -> P {
    let direction = h.sub(t);

    if direction.x.abs() > 1 || direction.y.abs() > 1 {
        P::new(direction.x.signum(), direction.y.signum())
    } else {
        P::new(0, 0)
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
