use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day16 { input })
}

struct Day16 {
    input: String,
}

struct State<'a> {
    position: &'a str,
    open_valves: Vec<&'a str>,
    released: usize,
    remaining_time: usize,
    rates: &'a HashMap<&'a str, usize>,
}

impl<'a> Debug for State<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "position:{}, released:{}, remaining_time:{}, open:{:?}",
            &self.position, self.released, self.remaining_time, self.open_valves
        )
    }
}

impl<'a> State<'a> {
    fn releases(&self) -> usize {
        self.open_valves.iter().map(|valve| self.rates[valve]).sum()
    }
}

impl Day for Day16 {
    fn part1(&self) -> String {
        let mut links: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut rates: HashMap<&str, usize> = HashMap::new();
        self.input.lines().for_each(|line| {
            let name = line.split(' ').nth(1).unwrap();
            let rate: usize = line.split(&['=', ';']).nth(1).unwrap().parse().unwrap();
            let connections = if line.find("valves").is_some() {
                line.split_once("valves ")
                    .unwrap()
                    .1
                    .split(", ")
                    .collect_vec()
            } else {
                vec![line.split(' ').last().unwrap()]
            };
            links.insert(name, connections);
            rates.insert(name, rate);
        });

        let important_valves = rates
            .iter()
            .filter(|(v, r)| **v == "AA" || **r > 0)
            .map(|(v, _)| *v)
            .collect_vec();
        let distances = apsp(&links, &important_valves);

        let start = State {
            position: "AA",
            open_valves: vec!["AA"],
            remaining_time: 30,
            released: 0,
            rates: &rates,
        };

        best_gain(&distances, start).to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn best_gain(distances: &HashMap<&str, Vec<(&str, usize)>>, state: State) -> usize {
    //println!("{:?}", state);
    distances[&state.position]
        .iter()
        .filter(|(destination, distance)| {
            !(state.open_valves.contains(destination) || distance + 1 > state.remaining_time)
        })
        .map(|(dest, dist)| {
            let mut valves = state.open_valves.clone();
            valves.push(*dest);
            let new_state = State {
                position: *dest,
                remaining_time: state.remaining_time - (*dist + 1),
                released: state.released + state.releases() * (*dist + 1),
                open_valves: valves,
                ..state
            };
            best_gain(distances, new_state)
        })
        .max()
        .or_else(|| {
            let releases = state.releases() * state.remaining_time;
            Some(state.released + releases)
        })
        .unwrap()
}

fn apsp<'a>(
    links: &HashMap<&str, Vec<&str>>,
    main_nodes: &'a [&str],
) -> HashMap<&'a str, Vec<(&'a str, usize)>> {
    main_nodes
        .iter()
        .map(|a| {
            let v = main_nodes
                .iter()
                .filter(|b| a != *b)
                .map(|b| (*b, distance(*a, *b, links)))
                .collect_vec();
            (*a, v)
        })
        .collect()
}

fn distance(start: &str, goal: &str, links: &HashMap<&str, Vec<&str>>) -> usize {
    let mut visited = HashSet::new();
    let mut open = VecDeque::new();
    open.push_back((start, 0));
    while let Some((current, distance)) = open.pop_front() {
        if current == goal {
            return distance;
        }

        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        for &link in &links[&current] {
            open.push_back((link, distance + 1));
        }
    }

    unreachable!("no path found")
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day16::Day16;

    #[test]
    fn part1() {
        let day = Day16 {
            input: INPUT.to_string(),
        };
        assert_eq!(day.part1(), "1651");
    }

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
}
