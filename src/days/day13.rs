use std::cmp::Ordering;
use std::vec;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;

use crate::days::Day;

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
            Value::Integer(lhs_i) =>
                match other {
                    Value::Integer(rhs_i) =>
                        lhs_i.partial_cmp(rhs_i),
                    Value::List(_) => {
                        Value::List(vec![Value::Integer(*lhs_i)]).partial_cmp(other)
                    }
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
                Value::Integer(i) =>
                    self.partial_cmp(&Value::List(vec![Value::Integer(*i)]))
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
        s.1.0.iter().enumerate().filter(|(_, pair)| pair.lhs < pair.rhs).map(|(idx, _)| idx + 1).sum::<usize>().to_string()
    }

    fn part2(&self) -> String {
        let (_, s) = signal(&self.input).unwrap();
        let mut packets = s.0.into_iter().flat_map(|pair| vec![pair.rhs, pair.lhs]).collect_vec();
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

fn signal(s: &str) -> IResult<&str, Signal> {
    separated_list0(tag("\n\n"), pair)(s).map(|(rem, pairs)| (rem, Signal(pairs)))
}

fn pair(s: &str) -> IResult<&str, Pair> {
    tuple((packet, tag("\n"), packet))(s).map(|(rem, (lhs, _, rhs))| (rem, Pair { lhs, rhs }))
}

fn packet(s: &str) -> IResult<&str, Packet> {
    value(s).map(|(rem, value)| (rem, Packet(value)))
}

fn value(s: &str) -> IResult<&str, Value> {
    alt((int, list))(s)
}

fn int(s: &str) -> IResult<&str, Value> {
    i32(s).map(|(rem, v)| (rem, Value::Integer(v)))
}

fn list(s: &str) -> IResult<&str, Value> {
    tuple((tag("["),
           separated_list0(tag(","), value),
           tag("]")
    ))
        (s).map(|(rem, (_, values, _))| (rem, Value::List(values)))
}

#[cfg(test)]
mod test {
    use crate::days::day13::{int, list, packet, Packet, Pair, pair, signal, Signal, Value};

    #[test]
    fn parse_int() {
        assert_eq!(int("9"), Ok(("", Value::Integer(9))));
    }

    #[test]
    fn parse_list() {
        assert_eq!(list("[]"), Ok(("", Value::List(vec![]))));
    }

    #[test]
    fn parse_packet() {
        assert_eq!(packet("[]"), Ok(("", Packet(Value::List(vec![])))));
    }

    #[test]
    fn parse_pair() {
        assert_eq!(pair("[]\n[]\n"), Ok(("\n", Pair { lhs: Packet(Value::List(vec![])), rhs: Packet(Value::List(vec![])) })));
    }

    #[test]
    fn parse_signal() {
        assert_eq!(signal("[]\n[]\n"), Ok(("\n", Signal(vec![Pair { lhs: Packet(Value::List(vec![])), rhs: Packet(Value::List(vec![])) }]))));
    }
}