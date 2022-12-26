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
        let monkeys: HashMap<&str, &str> = self
            .input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .collect();

        let (left, _, right) = monkeys["root"].split(' ').collect_tuple().unwrap();
        let mut results: HashMap<&str, i64> = HashMap::new();
        let left_result = pre_solve2(&monkeys, &mut results, left);
        let right_result = pre_solve2(&monkeys, &mut results, right);

        if left_result.is_none() && right_result.is_none() {
            panic!("fuck^^")
        }

        if let Some(result) = left_result {
            solve2(&monkeys, &results, left, result)
        } else {
            solve2(&monkeys, &results, right, left_result.unwrap())
        }
            .to_string()
    }
}

fn solve2(
    monkeys: &HashMap<&str, &str>,
    results: &HashMap<&str, i64>,
    monkey: &str,
    needed: i64,
) -> i64 {
    if monkey == "humn" {
        return needed;
    }
    let stuff = monkeys[monkey];
    let (lhs, op, rhs) = stuff.split(' ').collect_tuple().unwrap();
    if let Some(value) = results.get(lhs) {
        let needed_from_right = match op {
            "+" => needed - *value,
            "-" => -(needed - *value),
            "*" => needed / *value,
            "/" => *value / needed,
            _ => panic!("unknown character"),
        };
        return solve2(monkeys, results, rhs, needed_from_right);
    }

    if let Some(value) = results.get(rhs) {
        let needed_from_left = match op {
            "+" => needed - *value,
            "-" => needed + *value,
            "*" => needed / *value,
            "/" => needed * *value,
            _ => panic!("unknown character"),
        };
        return solve2(monkeys, results, lhs, needed_from_left);
    }

    unreachable!()
}

fn pre_solve2<'a>(
    monkeys: &HashMap<&'a str, &'a str>,
    results: &mut HashMap<&'a str, i64>,
    monkey: &'a str,
) -> Option<i64> {
    if monkey == "humn" {
        return None;
    }
    let stuff = monkeys[monkey];
    let result = if let Some((lhs, op, rhs)) = stuff.split(' ').collect_tuple() {
        let lhs = pre_solve2(monkeys, results, lhs);
        let rhs = pre_solve2(monkeys, results, rhs);
        if lhs.is_none() || rhs.is_none() {
            return None;
        }

        match op {
            "+" => lhs.unwrap() + rhs.unwrap(),
            "-" => lhs.unwrap() - rhs.unwrap(),
            "*" => lhs.unwrap() * rhs.unwrap(),
            "/" => lhs.unwrap() / rhs.unwrap(),
            _ => panic!("unknown character"),
        }
    } else {
        stuff.parse().unwrap()
    };

    results.insert(monkey, result);
    Some(result)
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
            _ => panic!("unknown character"),
        }
    } else {
        stuff.parse().unwrap()
    }
}
