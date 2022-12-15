use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day15 {
        input,
        part1_row: 2000000,
    })
}

struct Day15 {
    input: String,
    part1_row: i32,
}

type P = (i32, i32);

impl Day for Day15 {
    fn part1(&self) -> String {
        let sb: Vec<(P, P)> = self
            .input
            .lines()
            .map(|line| line.split(&[',', ' ', '=', ':']).collect_vec())
            .map(|v| {
                (
                    (v[3].parse().unwrap(), v[6].parse().unwrap()),
                    (v[13].parse().unwrap(), v[16].parse().unwrap()),
                )
            })
            .inspect(|x| println!("{:?}", x))
            .collect_vec();

        let max_distance = sb.iter().map(|(s, b)| manhattan(s, b)).max().unwrap();
        let sensor_x_min = sb.iter().map(|(s, _)| s.0).min().unwrap();
        let sensor_x_max = sb.iter().map(|(s, _)| s.0).max().unwrap();

        (sensor_x_min - max_distance..=sensor_x_max + max_distance)
            .filter(|&x| {
                sb.iter().any(|(s, b)| {
                    let spot = (x, self.part1_row);
                    let d_field = manhattan(s, &spot);
                    let d_closest_beacon = manhattan(s, b);
                    let too_close_to_signal = d_field <= d_closest_beacon;
                    let is_on_beacon = spot == *b;
                    too_close_to_signal && !is_on_beacon
                })
            })
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

fn manhattan(p1: &P, p2: &P) -> i32 {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day15::{Day15, manhattan};

    #[test]
    fn part1() {
        assert_eq!(
            Day15 {
                input: INPUT.to_string(),
                part1_row: 10,
            }
                .part1(),
            "26"
        );
    }

    #[test]
    fn distance() {
        assert_eq!(manhattan(&(3, 5), &(-2, 9)), 9);
    }

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
}
