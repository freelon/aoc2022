use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day25 { input })
}

struct Day25 {
    input: String,
}

fn convert_from_snafu(input: &str) -> i64 {
    input
        .chars()
        .map(|c| digit(c))
        .rev()
        .enumerate()
        .map(|(i, d)| 5i64.pow(i as u32) * d)
        .sum()
}

fn digit(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }
}

impl Day for Day25 {
    fn part1(&self) -> String {
        self.input
            .lines()
            .fold("0".to_string(), |acc, item| add_snafu(&acc, item))
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn add_snafu(a: &str, b: &str) -> String {
    println!("\nsnafu add {a}, {b}");
    let a = a.to_string();
    let b = b.to_string();
    let mut v = vec![a, b];
    v.sort_by_key(|s| s.len());

    let mut b = v.pop().unwrap();
    let mut a = v.pop().unwrap();

    while a.len() < b.len() {
        a.insert(0, '0');
    }

    println!("{a} {b}");

    let mut carry = 0;
    let mut result = String::new();
    for _i in (0..a.len()).rev() {
        let ca = a.pop().unwrap();
        let va = digit(ca);
        let cb = b.pop().unwrap();
        let vb = digit(cb);

        println!("{ca}|{va} - {cb}|{vb}   carry: {carry}");
        let mut sum = va + vb;
        println!("sum:  {sum}");
        sum += carry;
        println!("sum': {sum}");

        if sum > 2 {
            sum -= 5;
            carry = 1;
        } else if sum < -2 {
            sum += 5;
            carry = -1;
        } else {
            carry = 0;
        }

        let d = snafu(sum);
        println!("final sum: {sum}, d: {d}");
        result.insert(0, d);
    }

    if carry == -1 {
        result.insert(0, '-');
    } else if carry == 1 {
        result.insert(0, '1');
    }

    println!("result {result}");
    //  if result.chars().next().unwrap() == '0' {
//        result.remove(0);
    //}
    result
}

fn to_snafu(n: i64) -> String {
    println!("\nn: {n}");
    let mut remainder = n;
    let mut i = 0;

    let mut digits = String::new();

    let mut increment = false;
    while remainder > 0 {
        let v = 5i64.pow(i + 1);
        println!("remainder: {remainder}, v: {v}, i: {i}, increment: {increment}");
        let mut x = remainder % v;
        if increment {
            x += 1;
            increment = false;
        }
        println!("x: {x}");

        if x > 2 {
            x -= 5;
            increment = true;
        }
        println!("x': {x}");

        remainder -= 5i64.pow(i) * x;

        let s = snafu(x);
        digits.push(s);
        i += 1;
    }

    digits.chars().rev().collect()
}

fn snafu(x: i64) -> char {
    match x {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => {
            eprintln!("{x} doesn't fit");
            panic!("unknown char")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day25::{add_snafu, convert_from_snafu, Day25, to_snafu};

    #[test]
    fn snafu_to_10() {
        assert_eq!(convert_from_snafu("1=-0-2"), 1747);
        assert_eq!(convert_from_snafu("12111"), 906);
        assert_eq!(convert_from_snafu("2=0="), 198);
        assert_eq!(convert_from_snafu("21"), 11);
        assert_eq!(convert_from_snafu("2=01"), 201);
        assert_eq!(convert_from_snafu("111"), 31);
        assert_eq!(convert_from_snafu("20012"), 1257);
        assert_eq!(convert_from_snafu("112"), 32);
        assert_eq!(convert_from_snafu("1=-1="), 353);
        assert_eq!(convert_from_snafu("1-12"), 107);
        assert_eq!(convert_from_snafu("12"), 7);
        assert_eq!(convert_from_snafu("1="), 3);
        assert_eq!(convert_from_snafu("122"), 37);
    }

    #[test]
    fn decimal_to_snafu() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn add() {
        assert_eq!(add_snafu("1", "1"), "02");
        assert_eq!(add_snafu("1=", "1="), "011");
        // 3 + 4 = 7
        assert_eq!(add_snafu("1=", "1-"), "012");
        // 2 + 8 = 10
        assert_eq!(add_snafu("2", "2="), "20");
    }

    #[test]
    fn example() {
        let day = Day25 {
            input: INPUT.to_string(),
        };
        assert_eq!(day.part1(), "2=-1=0");
    }

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
}
