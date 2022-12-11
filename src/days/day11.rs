use std::collections::VecDeque;

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day11 { input })
}

struct Day11 {
    input: String,
}

impl Day for Day11 {
    fn part1(&self) -> String {
        let mut monkeys = read(&self.input);
        println!("{:?}", monkeys);
        for _ in 0..20 {
            for m in 0..monkeys.len() {
                while let Some((target, number)) = monkeys[m].play() {
                    println!("{m} throws {number} to {target}");
                    monkeys[target].items.push_back(number);
                }
            }
        }

        let mut inspections = monkeys.iter().map(|m| m.inspections).collect_vec();
        inspections.sort();
        inspections.reverse();
        (inspections[0] * inspections[1]).to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn read(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| Monkey::new(block))
        .collect_vec()
}

#[derive(Debug, Default)]
struct Monkey {
    items: VecDeque<i64>,
    operation: (String, String, String),
    test_divisor: i64,
    target_false: usize,
    target_true: usize,
    inspections: usize,
}

impl Monkey {
    pub(crate) fn play(&mut self) -> Option<(usize, i64)> {
        if let Some(item) = self.items.pop_front() {
            self.inspections += 1;

            let level_after_inspection = {
                let lhs: i64 = self.operation.0.parse::<i64>().unwrap_or(item);
                let rhs: i64 = self.operation.2.parse::<i64>().unwrap_or(item);
                let result = match self.operation.1.chars().next().unwrap() {
                    '*' => lhs * rhs,
                    '+' => lhs + rhs,
                    _ => panic!("unknown operator")
                };
                result
            };

            let level = level_after_inspection / 3;

            let target = if level % self.test_divisor == 0 {
                self.target_true
            } else {
                self.target_false
            };

            Some((target, level))
        } else {
            None
        }
    }

    fn new(input: &str) -> Self {
        let lines = input.lines().collect_vec();
        let items: VecDeque<i64> = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let op = lines[2].split(' ').collect_vec();
        let operation = (
            op[op.len() - 3].to_string(),
            op[op.len() - 2].to_string(),
            op[op.len() - 1].to_string(),
        );

        let test_divisor: i64 = lines[3]
            .split_once("divisible by ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let target_true: usize = lines[4]
            .split_once("throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let target_false: usize = lines[5]
            .split_once("throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        Monkey {
            items,
            operation,
            test_divisor,
            target_false,
            target_true,
            ..Monkey::default()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day11::Day11;

    #[test]
    fn part1() {
        assert_eq!(Day11 { input: EXAMPLE.to_string() }.part1(), "10605");
    }

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}