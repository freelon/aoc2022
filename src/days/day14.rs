use std::collections::HashMap;

use itertools::Itertools;

use Type::*;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day14 { input })
}

struct Day14 {
    input: String,
}

#[derive(Eq, PartialEq)]
enum Type {
    Sand,
    Rock,
}

type P = (i32, i32);

impl Day for Day14 {
    fn part1(&self) -> String {
        let mut map = self.load();
        play(&mut map);
        map.values().filter(|it| **it == Sand).count().to_string()
    }

    fn part2(&self) -> String {
        let mut map = self.load();
        play2(&mut map);
        map.values().filter(|it| **it == Sand).count().to_string()
    }
}

fn play(map: &mut HashMap<P, Type>) {
    let depth = *map.keys().map(|(_, y)| y).max().unwrap();
    'outer: loop {
        let mut s = (500, 0);
        loop {
            let d = directions(s);
            let d = d.into_iter().find(|next| map.get(next).is_none());
            if let Some(next) = d {
                s = next;

                if s.1 > depth {
                    // nowhere else to go
                    break 'outer;
                }
            } else {
                map.insert(s, Sand);
                continue 'outer;
            }
        }
    }
}

fn play2(map: &mut HashMap<P, Type>) {
    let depth = *map.keys().map(|(_, y)| y).max().unwrap();
    'outer: loop {
        let mut s = (500, 0);
        loop {
            let d = directions(s);
            let d = d
                .into_iter()
                .find(|next| map.get(next).is_none() && next.1 < depth + 2);
            if let Some(next) = d {
                s = next;
            } else {
                map.insert(s, Sand);

                if s == (500, 0) {
                    break 'outer;
                }

                continue 'outer;
            }
        }
    }
}

fn directions(p: P) -> Vec<P> {
    vec![(p.0, p.1 + 1), (p.0 - 1, p.1 + 1), (p.0 + 1, p.1 + 1)]
}

impl Day14 {
    fn load(&self) -> HashMap<P, Type> {
        let mut map = HashMap::<P, Type>::new();
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
                    map.insert((xc, yc), Rock);
                    xc += dx;
                    yc += dy;
                }
                map.insert((xc, yc), Rock);
            }
        }
        map
    }
}

#[allow(dead_code)]
fn print_map(map: &HashMap<P, Type>) {
    let x_min = map.keys().min_by_key(|it| it.0).unwrap().0;
    let x_max = map.keys().max_by_key(|it| it.0).unwrap().0;
    let y_min = map.keys().min_by_key(|it| it.1).unwrap().1;
    let y_max = map.keys().max_by_key(|it| it.1).unwrap().1;

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = match map.get(&(x, y)) {
                Some(t) => match t {
                    Sand => 'o',
                    Rock => '#',
                },
                None => '.',
            };
            print!("{c}");
        }
        println!();
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
