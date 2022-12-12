use std::collections::VecDeque;
use std::time::{Duration, Instant};

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
        let (_, mut monkeys) = time(|| read(&self.input), "input 1");
        let reduction = |level| level / 3;
        for _ in 0..20 {
            for m in 0..monkeys.len() {
                while let Some((target, number)) = monkeys[m].play(reduction) {
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
        let (_, mut monkeys) = time(|| read(&self.input), "input 1");
        let all: i64 = monkeys.iter().map(|m| m.test_divisor).product();
        let reduction = |level| level % all;
        for _ in 0..10000 {
            for m in 0..monkeys.len() {
                while let Some((target, number)) = monkeys[m].play(reduction) {
                    monkeys[target].items.push_back(number);
                }
            }
        }

        let mut inspections = monkeys.iter().map(|m| m.inspections).collect_vec();
        inspections.sort();
        inspections.reverse();
        (inspections[0] * inspections[1]).to_string()
    }
}

fn read(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::new).collect_vec()
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: (Operator, Operand),
    test_divisor: i64,
    target_false: usize,
    target_true: usize,
    inspections: usize,
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(i64),
}

impl Operand {
    fn value(&self, item_value: i64) -> i64 {
        match self {
            Operand::Old => item_value,
            Operand::Value(v) => *v,
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiplication,
}

impl Monkey {
    pub(crate) fn play<F>(&mut self, reduction: F) -> Option<(usize, i64)>
        where
            F: Fn(i64) -> i64,
    {
        if let Some(item) = self.items.pop_front() {
            self.inspections += 1;

            let level_after_inspection = {
                let rhs: i64 = self.operation.1.value(item);
                match self.operation.0 {
                    Operator::Multiplication => item * rhs,
                    Operator::Add => item + rhs,
                }
            };

            let level = reduction(level_after_inspection);

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
            match op[op.len() - 2].chars().next().unwrap() {
                '*' => Operator::Multiplication,
                '+' => Operator::Add,
                _ => panic!("unknown operator"),
            },
            op[op.len() - 1]
                .parse::<i64>()
                .map(Operand::Value)
                .unwrap_or(Operand::Old),
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
            inspections: 0,
        }
    }
}

fn time<F, T>(f: F, name: &str) -> (Duration, T)
    where
        F: Fn() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = Instant::now().duration_since(start);
    println!("Duration of {name}: {:?}", duration);
    (duration, result)
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day11::Day11;

    #[test]
    fn part1() {
        assert_eq!(
            Day11 {
                input: EXAMPLE.to_string()
            }
                .part1(),
            "10605"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Day11 {
                input: EXAMPLE.to_string()
            }
                .part2(),
            "2713310158"
        );
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
