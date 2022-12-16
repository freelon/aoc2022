use std::collections::HashMap;

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day16 { input })
}

struct Day16 {
    input: String,
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
        format!("")
    }

    fn part2(&self) -> String {
        format!("")
    }
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
