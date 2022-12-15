use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day15 {
        input,
        part1_row: 2000000,
        max: 4194304, //4000000,
    })
}

struct Day15 {
    input: String,
    part1_row: i64,
    max: i64,
}

type P = (i64, i64);

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
            .collect_vec();

        let sensor_and_distances = sb.iter().map(|(a, b)| (*a, manhattan(a, b))).collect_vec();
        let beacons = sb.iter().map(|(_, b)| *b).collect_vec();

        let spot = rec(
            (0, 0),
            (self.max, self.max),
            &sensor_and_distances,
            &beacons,
        )
            .expect("there must be a solution");

        (spot.0 * 4000000 + spot.1).to_string()
    }
}

const MAX_COORDINATE: i64 = 4000000;

fn rec(from: P, to: P, sensors_and_distances: &[(P, i64)], beacons: &[P]) -> Option<P> {
    if from == to {
        return if sensors_and_distances.iter().all(|(signal, d)| {
            let spot = from;
            let d_field = manhattan(signal, &spot);
            d_field > *d
        }) && !beacons.contains(&from)
        {
            Some(from)
        } else {
            None
        };
    }

    let corners = [from, (from.0, to.1), (to.0, from.1), to];

    if corners
        .iter()
        .all(|corner| corner.0 > MAX_COORDINATE || corner.1 > MAX_COORDINATE)
    {
        return None;
    }

    if all_corners_covered_by_one_sensor(sensors_and_distances, &corners) {
        return None;
    }

    let m = ((from.0 + to.0) / 2, (from.1 + to.1) / 2);
    if let Some(result) = rec(from, m, sensors_and_distances, beacons) {
        return Some(result);
    }
    if let Some(result) = rec(
        (m.0 + 1, from.1),
        (to.0, m.1),
        sensors_and_distances,
        beacons,
    ) {
        return Some(result);
    }
    if let Some(result) = rec(
        (from.0, m.1 + 1),
        (m.0, to.1),
        sensors_and_distances,
        beacons,
    ) {
        return Some(result);
    }
    if let Some(result) = rec((m.0 + 1, m.1 + 1), to, sensors_and_distances, beacons) {
        return Some(result);
    }

    None
}

fn all_corners_covered_by_one_sensor(sensors_and_distances: &[(P, i64)], corners: &[P; 4]) -> bool {
    sensors_and_distances.iter().any(|(sensor, d_sb)| {
        corners
            .iter()
            .all(|corner| manhattan(sensor, corner) <= *d_sb)
    })
}

fn manhattan(p1: &P, p2: &P) -> i64 {
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
                max: 20,
            }
                .part1(),
            "26"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Day15 {
                input: INPUT.to_string(),
                part1_row: 10,
                max: 32,
            }
                .part2(),
            "56000011"
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
