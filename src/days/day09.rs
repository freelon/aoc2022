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
        let mut initial_state = State::default();
        initial_state.visited.insert(P::new(0, 0));
        let result = self
            .input
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

                state.h = state.h.add(move_head);

                let move_tail = tail_move_direction(state.h, state.t);
                state.t = state.t.add(move_tail);

                state.visited.insert(state.t);

                state
            });

        result.visited.len().to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

#[derive(Default)]
struct State {
    h: P,
    t: P,
    visited: HashSet<P>,
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
