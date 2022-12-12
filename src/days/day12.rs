use std::collections::{HashMap, VecDeque};

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day12 { input })
}

type Point = (i32, i32);
type Height = u8;
type Map = HashMap<Point, Height>;

struct Day12 {
    input: String,
}

impl Day for Day12 {
    fn part1(&self) -> String {
        let (map, start, stop) = read(&self.input);
        shortest_paths(&map, stop)[&start].to_string()
    }

    fn part2(&self) -> String {
        let (map, _, stop) = read(&self.input);

        shortest_paths(&map, stop)
            .iter()
            .filter(|(p, _)| map[p] == 0)
            .map(|(_, distance)| distance)
            .min()
            .unwrap()
            .to_string()
    }
}

fn read(input: &str) -> (Map, Point, Point) {
    let map: HashMap<Point, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    let start = *map.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let goal = *map.iter().find(|(_, v)| **v == 'E').unwrap().0;

    let map: Map = map
        .into_iter()
        .map(|(p, v)| {
            let h = match v {
                'a'..='z' => v as u8 - b'a',
                'S' => 0,
                'E' => b'z' - b'a',
                _ => panic!("unexpected input character"),
            };
            (p, h)
        })
        .collect();
    (map, start, goal)
}

fn can_go_to_target(map: &Map, target: Point) -> Vec<Point> {
    let own_height = map[&target];
    vec![
        (target.0, target.1 + 1),
        (target.0, target.1 - 1),
        (target.0 + 1, target.1),
        (target.0 - 1, target.1),
    ]
        .into_iter()
        .filter(|p| {
            if let Some(h) = map.get(p) {
                *h + 1 >= own_height
            } else {
                false
            }
        })
        .collect()
}

fn shortest_paths(map: &Map, stop: Point) -> HashMap<Point, usize> {
    let mut visited: HashMap<Point, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((stop, 0));
    while let Some((next, distance)) = queue.pop_front() {
        if visited.contains_key(&next) {
            continue;
        }

        visited.insert(next, distance);

        let neighbors = can_go_to_target(map, next);
        for n in neighbors {
            queue.push_back((n, distance + 1));
        }
    }

    visited
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day12::Day12;

    #[test]
    fn part1() {
        assert_eq!(
            Day12 {
                input: INPUT.to_string()
            }
                .part1(),
            "31"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Day12 {
                input: INPUT.to_string()
            }
                .part2(),
            "29"
        );
    }

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
}
