use std::cmp::Ordering;
use std::vec;

use itertools::Itertools;

use crate::days::Day;
use crate::days::day13::parser::signal;

mod parser;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day13 { input })
}

#[derive(Debug, PartialEq)]
struct Signal(Vec<Pair>);

#[derive(Debug, PartialEq)]
struct Pair {
    lhs: Packet,
    rhs: Packet,
}

#[derive(Debug, PartialEq, Clone)]
struct Packet(Value);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Packet {}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Integer(i32),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Value::Integer(lhs_i) => match other {
                Value::Integer(rhs_i) => lhs_i.partial_cmp(rhs_i),
                Value::List(_) => Value::List(vec![Value::Integer(*lhs_i)]).partial_cmp(other),
            },
            Value::List(lhs) => match other {
                Value::List(rhs) => {
                    for (l, r) in lhs.iter().zip(rhs) {
                        let cmp = l.partial_cmp(r);
                        if cmp == Some(Ordering::Equal) {
                            continue;
                        }
                        return cmp;
                    }

                    lhs.len().partial_cmp(&rhs.len())
                }
                Value::Integer(i) => self.partial_cmp(&Value::List(vec![Value::Integer(*i)])),
            },
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Value {}

#[derive(Debug)]
struct Day13 {
    input: String,
}

impl Day for Day13 {
    fn part1(&self) -> String {
        let s = signal(&self.input).unwrap();
        s.1.0
            .iter()
            .enumerate()
            .filter(|(_, pair)| pair.lhs < pair.rhs)
            .map(|(idx, _)| idx + 1)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let (_, s) = signal(&self.input).unwrap();
        let mut packets =
            s.0.into_iter()
                .flat_map(|pair| vec![pair.rhs, pair.lhs])
                .collect_vec();
        let p1 = Packet(Value::List(vec![Value::List(vec![Value::Integer(2)])]));
        let p2 = Packet(Value::List(vec![Value::List(vec![Value::Integer(6)])]));
        packets.push(p1.clone());
        packets.push(p2.clone());
        packets.sort();

        let i1 = packets.iter().find_position(|i| **i == p1).unwrap().0 + 1;
        let i2 = packets.iter().find_position(|i| **i == p2).unwrap().0 + 1;

        (i1 * i2).to_string()
    }
}
