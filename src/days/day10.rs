use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day10 { input })
}

struct Day10 {
    input: String,
}

impl Day for Day10 {
    fn part1(&self) -> String {
        self.input
            .lines()
            .flat_map(|line| {
                if line.starts_with("noop") {
                    vec![0]
                } else {
                    let (_, v) = line.split_once(' ').unwrap();
                    let v: i32 = v.parse().unwrap();
                    vec![0, v]
                }
            })
            .fold((vec![], 1), |(mut result, mut x), v| {
                x += v;
                result.push(x);
                (result, x)
            })
            .0
            .into_iter()
            // value during cycle x is v[c-2]
            // -1 because the vec is 0 based
            // -1 because vec[y] stores what's in the register _after_ cycle y
            .enumerate()
            .skip(18)
            .step_by(40)
            .map(|(x, v)| (x as i32 + 2) * v)
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        let values_after_cycle = self
            .input
            .lines()
            .flat_map(|line| {
                if line.starts_with("noop") {
                    vec![0]
                } else {
                    let (_, v) = line.split_once(' ').unwrap();
                    let v: i32 = v.parse().unwrap();
                    vec![0, v]
                }
            })
            .fold((vec![1], 1), |(mut result, mut x), v| {
                x += v;
                result.push(x);
                (result, x)
            })
            .0;

        let pixels: String = (0..240)
            .zip(values_after_cycle)
            .map(|(cycle, sprite_x)| {
                let crt_x = cycle % 40;
                if (sprite_x - crt_x).abs() <= 1 {
                    '█'
                } else {
                    ' '
                }
            })
            .collect();

        for line in 0..6 {
            println!("{}", pixels.chars().skip(40 * line).take(40).collect::<String>());
        }

        "EHBZLRJR".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day10::Day10;

    #[test]
    fn foo() {
        assert_eq!(
            Day10 {
                input: EXAMPLE.to_string()
            }
                .part1(),
            "13140"
        );
    }

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
}
