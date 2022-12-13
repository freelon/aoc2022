use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::eof;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, separated_pair};

use crate::days::day13::{Packet, Pair, Signal, Value};

pub(super) fn signal(s: &str) -> IResult<&str, Signal> {
    pair(separated_list0(tag("\n\n"), pairs), eof)(s.trim()).map(|(rem, (pairs, _))| (rem, Signal(pairs)))
}

fn pairs(s: &str) -> IResult<&str, Pair> {
    separated_pair(packet, tag("\n"), packet)(s).map(|(rem, (lhs, rhs))| (rem, Pair { lhs, rhs }))
}

fn packet(s: &str) -> IResult<&str, Packet> {
    list(s).map(|(rem, value)| (rem, Packet(value)))
}

fn value(s: &str) -> IResult<&str, Value> {
    alt((int, list))(s)
}

fn int(s: &str) -> IResult<&str, Value> {
    i32(s).map(|(rem, v)| (rem, Value::Integer(v)))
}

fn list(s: &str) -> IResult<&str, Value> {
    delimited(tag("["), separated_list0(tag(","), value), tag("]"))(s)
        .map(|(rem, values)| (rem, Value::List(values)))
}

#[cfg(test)]
mod test {
    use crate::days::day13::parser::{int, list, packet, pairs, signal};

    use super::{Packet, Pair, Signal, Value};

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
        assert_eq!(
            pairs("[]\n[]\n"),
            Ok((
                "\n",
                Pair {
                    lhs: Packet(Value::List(vec![])),
                    rhs: Packet(Value::List(vec![])),
                }
            ))
        );
    }

    #[test]
    fn parse_signal() {
        assert_eq!(
            signal("[]\n[]"),
            Ok((
                "",
                Signal(vec![Pair {
                    lhs: Packet(Value::List(vec![])),
                    rhs: Packet(Value::List(vec![])),
                }])
            ))
        );
    }

    #[test]
    fn parse_bad_packet_start() {
        let error = packet("5").err();
        assert!(
            error.is_some(),
            "doesn't start with a '[', so an error should be returned"
        );
    }
}
