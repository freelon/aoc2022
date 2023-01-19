use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day24 { input })
}

struct Day24 {
    input: String,
}

type P = (i32, i32);

type Blizzards = HashMap<P, Vec<char>>;

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    e: P,
    blizzards: Vec<(P, char)>,
    x_wall_left: i32,
    x_wall_right: i32,
    y_wall_top: i32,
    y_wall_bottom: i32,
}

fn advance(
    blizzards: &Blizzards,
    x_wall_left: i32,
    x_wall_right: i32,
    y_wall_top: i32,
    y_wall_bottom: i32,
) -> Blizzards {
    blizzards
        .iter()
        .flat_map(|(p, cs)| cs.iter().map(|c| (*p, *c)))
        .map(|((x, y), c)| match c {
            '<' => {
                if x == x_wall_left + 1 {
                    ((x_wall_right - 1, y), c)
                } else {
                    ((x - 1, y), c)
                }
            }
            '>' => {
                if x == x_wall_right - 1 {
                    ((x_wall_left + 1, y), c)
                } else {
                    ((x + 1, y), c)
                }
            }
            'v' => {
                if y == y_wall_bottom - 1 {
                    ((x, y_wall_top + 1), c)
                } else {
                    ((x, y + 1), c)
                }
            }
            '^' => {
                if y == y_wall_top + 1 {
                    ((x, y_wall_bottom - 1), c)
                } else {
                    ((x, y - 1), c)
                }
            }
            _ => unreachable!("bitch!"),
        })
        .into_group_map()
}

fn valid(
    e: P,
    blizzards: &Blizzards,
    x_wall_left: i32,
    x_wall_right: i32,
    y_wall_top: i32,
    y_wall_bottom: i32,
) -> bool {
    if e.0 <= x_wall_left
        || e.0 >= x_wall_right
        || (e.1 <= y_wall_top && !(e.0 == 1))
        || (e.1 >= y_wall_bottom && !(e.0 == x_wall_right - 1))
    {
        return false;
    }

    !blizzards.contains_key(&e)
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
            .into_group_map();
        let x_wall_left = 0;
        let x_wall_right = (self.input.lines().next().unwrap().len() - 1) as i32;
        let y_wall_top = 0;
        let y_wall_bottom = (self.input.lines().count() - 1) as i32;

        let goal = (x_wall_right - 1, y_wall_bottom);

        let mut open: VecDeque<(P, usize)> = VecDeque::new();
        let mut visited: HashSet<(P, usize)> = HashSet::new();

        open.push_back(((1, 0), 0));

        let mut blizzards_at: Vec<Blizzards> = vec![start_blizzards];

        while let Some((e, time)) = open.pop_front() {
            if visited.contains(&(e, time)) {
                continue;
            }

            if e == goal {
                return format!("{time}");
            }

            while blizzards_at.len() < time + 2 {
                let b = advance(
                    blizzards_at.last().unwrap(),
                    x_wall_left,
                    x_wall_right,
                    y_wall_top,
                    y_wall_bottom,
                );
                blizzards_at.push(b);
            }

            let current_blizzards = blizzards_at.get(time + 1).unwrap();

            for delta in &[(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
                let e = add(delta, &e);

                if valid(
                    e,
                    &current_blizzards,
                    x_wall_left,
                    x_wall_right,
                    y_wall_top,
                    y_wall_bottom,
                ) {
                    open.push_back((e, time + 1));
                }
            }

            visited.insert((e, time));
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
