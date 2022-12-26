use std::collections::HashSet;
use std::ops::Deref;

use rustc_hash::FxHashMap;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day23 { input })
}

struct Day23 {
    input: String,
}

type P = (i32, i32);
type EmptyChecker = fn(&[bool; 8]) -> bool;
type DirectionProposer = fn(&P) -> P;

impl Day for Day23 {
    fn part1(&self) -> String {
        let mut elves: HashSet<P> = self
            .input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| (x as i32, y as i32))
            })
            .collect();

        let mut proposed_moves: FxHashMap<P, usize> = FxHashMap::default();
        let mut target_directions: Vec<(Box<EmptyChecker>, Box<DirectionProposer>)> = vec![
            (Box::new(north_empty), Box::new(north)),
            (Box::new(south_empty), Box::new(south)),
            (Box::new(west_empty), Box::new(west)),
            (Box::new(east_empty), Box::new(east)),
        ];
        for _ in 0..10 {
            proposed_moves.clear();
            elves.iter().for_each(|elf| {
                let target = Self::target_of(&elves, &target_directions, elf);

                if let Some(p) = target {
                    *proposed_moves.entry(p).or_insert(0) += 1;
                }
            });

            elves = elves
                .iter()
                .map(|elf| {
                    let target = Self::target_of(&elves, &target_directions, elf);

                    if let Some(target) = target {
                        if proposed_moves.get(&target).unwrap() == &1 {
                            return target;
                        }
                    }

                    *elf
                })
                .collect();

            let f1 = target_directions.remove(0);
            target_directions.push(f1);
        }

        let x_min = elves.iter().min_by_key(|elf| elf.0).unwrap().0;
        let x_max = elves.iter().max_by_key(|elf| elf.0).unwrap().0;
        let y_min = elves.iter().min_by_key(|elf| elf.1).unwrap().1;
        let y_max = elves.iter().max_by_key(|elf| elf.1).unwrap().1;

        let number_of_points = (x_max - x_min + 1) * (y_max - y_min + 1);
        (number_of_points - elves.len() as i32).to_string()
    }

    fn part2(&self) -> String {
        let mut elves: HashSet<P> = self
            .input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| (x as i32, y as i32))
            })
            .collect();

        let mut proposed_moves: FxHashMap<P, usize> = FxHashMap::default();
        let mut target_directions: Vec<(Box<EmptyChecker>, Box<DirectionProposer>)> = vec![
            (Box::new(north_empty), Box::new(north)),
            (Box::new(south_empty), Box::new(south)),
            (Box::new(west_empty), Box::new(west)),
            (Box::new(east_empty), Box::new(east)),
        ];
        for round in 1..(i32::MAX) {
            proposed_moves.clear();
            let targeted_elves: Vec<_> = elves
                .iter()
                .map(|elf| {
                    let target = Self::target_of(&elves, &target_directions, elf);

                    if let Some(p) = target {
                        (*elf, p)
                    } else {
                        (*elf, *elf)
                    }
                })
                .collect();

            targeted_elves.iter().for_each(|&(_from, to)| *proposed_moves.entry(to).or_insert(0) += 1);

            elves = targeted_elves
                .into_iter()
                .map(|(elf, target)| {
                    if let Some(1) = proposed_moves.get(&target) {
                        target
                    } else {
                        elf
                    }
                })
                .collect();

            if proposed_moves.is_empty() {
                return round.to_string();
            }

            let f1 = target_directions.remove(0);
            target_directions.push(f1);
        }

        unreachable!()
    }
}

#[allow(dead_code)]
fn print_map(elves: &HashSet<P>) {
    let x_min = elves.iter().min_by_key(|elf| elf.0).unwrap().0;
    let x_max = elves.iter().max_by_key(|elf| elf.0).unwrap().0;
    let y_min = elves.iter().min_by_key(|elf| elf.1).unwrap().1;
    let y_max = elves.iter().max_by_key(|elf| elf.1).unwrap().1;

    println!();
    for y in y_min - 2..=y_max + 2 {
        for x in x_min - 2..=x_max + 2 {
            let c = if elves.contains(&(x, y)) { '#' } else { '.' };
            print!("{c}");
        }
        println!();
    }
}

impl Day23 {
    fn target_of(
        elves: &HashSet<P>,
        target_directions: &[(Box<EmptyChecker>, Box<DirectionProposer>)],
        elf: &P,
    ) -> Option<P> {
        let n = [
            elves.contains(&(elf.0 - 1, elf.1 - 1)),
            elves.contains(&(elf.0, elf.1 - 1)),
            elves.contains(&(elf.0 + 1, elf.1 - 1)),
            elves.contains(&(elf.0 - 1, elf.1)),
            elves.contains(&(elf.0 + 1, elf.1)),
            elves.contains(&(elf.0 - 1, elf.1 + 1)),
            elves.contains(&(elf.0, elf.1 + 1)),
            elves.contains(&(elf.0 + 1, elf.1 + 1)),
        ];

        if !n.iter().any(|v| *v) {
            return None;
        }

        let target = target_directions
            .iter()
            .find(|f| f.deref().0(&n))
            .map(|f| f.deref().1(elf));
        target
    }
}

fn north(p: &P) -> P {
    (p.0, p.1 - 1)
}

fn south(p: &P) -> P {
    (p.0, p.1 + 1)
}

fn west(p: &P) -> P {
    (p.0 - 1, p.1)
}

fn east(p: &P) -> P {
    (p.0 + 1, p.1)
}

fn north_empty(map: &[bool; 8]) -> bool {
    !(map[0] || map[1] || map[2])
}

fn south_empty(map: &[bool; 8]) -> bool {
    !(map[5] || map[6] || map[7])
}

fn west_empty(map: &[bool; 8]) -> bool {
    !(map[0] || map[3] || map[5])
}

fn east_empty(map: &[bool; 8]) -> bool {
    !(map[2] || map[4] || map[7])
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day23::Day23;

    #[test]
    fn part1() {
        assert_eq!(
            Day23 {
                input: INPUT.to_string()
            }
                .part1(),
            "110"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Day23 {
                input: INPUT.to_string()
            }
                .part2(),
            "20"
        );
    }

    #[test]
    fn mini_example() {
        assert_eq!(
            Day23 {
                input: MINI_INPUT.to_string()
            }
                .part1(),
            "25"
        );
    }

    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    const MINI_INPUT: &str = ".....
..##.
..#..
.....
..##.
.....
";
}
