use std::collections::HashMap;

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day21 { input })
}

struct Day21 {
    input: String,
}

impl Day for Day21 {
    fn part1(&self) -> String {
        let monkeys: HashMap<&str, &str> = self
            .input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .collect();

        solve(&monkeys, "root").to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn solve(monkeys: &HashMap<&str, &str>, monkey: &str) -> i64 {
    let stuff = monkeys[monkey];
    if let Some((lhs, op, rhs)) = stuff.split(' ').collect_tuple() {
        let lhs = solve(monkeys, lhs);
        let rhs = solve(monkeys, rhs);
        match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            _ => panic!("unknown character")
        }
    } else {
        stuff.parse().unwrap()
    }
}
