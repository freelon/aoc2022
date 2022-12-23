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
        let mut target_directions: Vec<(Box<fn(&HashSet<P>, &P) -> bool>, Box<fn(&P) -> P>)> = vec![
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
                    let target = Self::target_of(&elves, &target_directions, &elf);

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

            print_map(&elves);
        }

        let x_min = elves.iter().min_by_key(|elf| elf.0).unwrap().0;
        let x_max = elves.iter().max_by_key(|elf| elf.0).unwrap().0;
        let y_min = elves.iter().min_by_key(|elf| elf.1).unwrap().1;
        let y_max = elves.iter().max_by_key(|elf| elf.1).unwrap().1;

        let number_of_points = (x_max - x_min + 1) * (y_max - y_min + 1);
        (number_of_points - elves.len() as i32).to_string()
    } // 4173 too high

    fn part2(&self) -> String {
        format!("")
    }
}

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
        target_directions: &Vec<(Box<fn(&HashSet<P>, &P) -> bool>, Box<fn(&P) -> P>)>,
        elf: &P,
    ) -> Option<P> {
        if target_directions.iter().all(|f| f.deref().0(elves, elf)) {
            return None;
        }

        let target = target_directions
            .iter()
            .find(|f| f.deref().0(&elves, elf))
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

fn north_empty(map: &HashSet<P>, p: &P) -> bool {
    !(map.contains(&(p.0 - 1, p.1 - 1))
        || map.contains(&(p.0, p.1 - 1))
        || map.contains(&(p.0 + 1, p.1 - 1)))
}

fn south_empty(map: &HashSet<P>, p: &P) -> bool {
    !(map.contains(&(p.0 - 1, p.1 + 1))
        || map.contains(&(p.0, p.1 + 1))
        || map.contains(&(p.0 + 1, p.1 + 1)))
}

fn west_empty(map: &HashSet<P>, p: &P) -> bool {
    !(map.contains(&(p.0 - 1, p.1 - 1))
        || map.contains(&(p.0 - 1, p.1))
        || map.contains(&(p.0 - 1, p.1 + 1)))
}

fn east_empty(map: &HashSet<P>, p: &P) -> bool {
    !(map.contains(&(p.0 + 1, p.1 - 1))
        || map.contains(&(p.0 + 1, p.1))
        || map.contains(&(p.0 + 1, p.1 + 1)))
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
