use std::fmt::{Debug, Display, Formatter, Write};

use itertools::Itertools;

use Type::*;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day14 { input })
}

struct Day14 {
    input: String,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Type {
    Sand,
    Rock,
    Air,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Sand => 'o',
            Rock => '#',
            Air => ' ',
        };
        f.write_char(c)
    }
}

type P = (i32, i32);

#[derive(Debug)]
struct Map<T: Copy> {
    inner: Vec<T>,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
}

impl<T> Map<T>
where
    T: Copy + Debug + Eq,
{
    fn index(&self, p: &P) -> usize {
        ((p.1 + self.y_offset) * self.width + (p.0 + self.x_offset)) as usize
    }

    fn insert(&mut self, p: P, t: T) {
        let idx = self.index(&p);
        self.inner[idx] = t;
    }

    fn index_to_position(&self, idx: usize) -> P {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x - self.x_offset, y - self.y_offset)
    }

    fn get(&self, p: &P) -> Option<&T> {
        if self.contains_pos(p) {
            Some(&self.inner[self.index(p)])
        } else {
            None
        }
    }

    fn contains_pos(&self, p: &P) -> bool {
        (p.0 + self.x_offset) >= 0
            && (p.0 + self.x_offset <= self.width)
            && (p.1 + self.y_offset) >= 0
            && (p.1 + self.y_offset <= self.height)
    }

    fn first_rev(&self, needle: T) -> Option<P> {
        self.inner
            .iter()
            .enumerate()
            .rev()
            .find(|(_, it)| **it == needle)
            .map(|(index, _)| self.index_to_position(index))
    }

    fn from(contents: &[(P, T)], default: T, x_margin: u32, y_margin: u32) -> Self {
        let x_margin = x_margin as i32;
        let y_margin = y_margin as i32;
        let x_min = contents.iter().min_by_key(|(it, _)| it.0).unwrap().0.0;
        let x_max = contents.iter().max_by_key(|(it, _)| it.0).unwrap().0.0;
        let y_min = contents.iter().min_by_key(|(it, _)| it.1).unwrap().0.1;
        let y_max = contents.iter().max_by_key(|(it, _)| it.1).unwrap().0.1;

        let width = (x_max - x_min + 1 + 2 * x_margin) as usize;
        let height = (y_max - y_min + 1 + 2 * y_margin) as usize;

        let inner = vec![default; width * height];
        let x_offset = -(x_min - x_margin);
        let y_offset = -(y_min - y_margin);

        let mut map = Map {
            inner,
            x_offset,
            y_offset,
            width: width as i32,
            height: height as i32,
        };

        for (p, t) in contents {
            map.insert(*p, *t);
        }
        map
    }
}

impl<T> Display for Map<T>
    where
        T: Display + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, t) in self.inner.iter().enumerate() {
            if idx % self.width as usize == 0 {
                f.write_char('\n')?;
            }
            f.write_fmt(format_args!("{t}"))?
        }
        Ok(())
    }
}

impl Day for Day14 {
    fn part1(&self) -> String {
        let mut result = String::new();
        for _ in 0..1000 {
            let mut map = self.load();
            result = play(&mut map).to_string();
        }
        result
    }
    fn part2(&self) -> String {
        let mut result = String::new();
        for _ in 0..1000 {
            let mut map = self.load();
            result = play2(&mut map).to_string();
        }
        result
    }
}

fn play(map: &mut Map<Type>) -> usize {
    let depth = map.first_rev(Rock).unwrap().1;
    let mut stopped_sand = 0;
    'outer: loop {
        let mut s = (500, 0);
        loop {
            let d = directions(s);
            let d = d.into_iter().find(|next| map.get(next) == Some(&Air));
            if let Some(next) = d {
                s = next;

                if s.1 >= depth {
                    // nowhere else to go
                    break 'outer;
                }
            } else {
                map.insert(s, Sand);
                stopped_sand += 1;
                continue 'outer;
            }
        }
    }
    stopped_sand
}

fn play2(map: &mut Map<Type>) -> usize {
    let lowest_rock = map.first_rev(Rock).unwrap().1;
    let mut stopped_sand = 0;
    'outer: loop {
        let mut s = (500, 0);
        loop {
            let d = directions(s);
            let d = d
                .into_iter()
                .find(|next| map.get(next) == Some(&Air) && next.1 < lowest_rock + 2);
            if let Some(next) = d {
                s = next;
            } else {
                map.insert(s, Sand);
                stopped_sand += 1;

                if s == (500, 0) {
                    break 'outer;
                }

                continue 'outer;
            }
        }
    }
    stopped_sand
}

fn directions(p: P) -> [P; 3] {
    [(p.0, p.1 + 1), (p.0 - 1, p.1 + 1), (p.0 + 1, p.1 + 1)]
}

impl Day14 {
    fn load(&self) -> Map<Type> {
        let mut start = vec![];
        for line in self.input.lines() {
            let coordinates: Vec<P> = line
                .split(" -> ")
                .map(|co| co.split_once(',').unwrap())
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .collect_vec();

            for ((xa, ya), (xb, yb)) in coordinates.into_iter().tuple_windows() {
                let dx = (xb - xa).signum();
                let dy = (yb - ya).signum();

                let mut xc = xa;
                let mut yc = ya;
                while (xc, yc) != (xb, yb) {
                    start.push(((xc, yc), Rock));
                    xc += dx;
                    yc += dy;
                }
                start.push(((xc, yc), Rock));
            }
        }
        Map::from(&start, Air, 500, 20)
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day14::Day14;

    #[test]
    fn part1() {
        assert_eq!(
            Day14 {
                input: INPUT.to_string()
            }
                .part1(),
            "24"
        );
    }

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
