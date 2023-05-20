use std::collections::VecDeque;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day20 { input })
}

struct Day20 {
    input: String,
}

impl Day for Day20 {
    fn part1(&self) -> String {
        let mut v: VecDeque<(usize, i32)> = self
            .input
            .lines()
            .enumerate()
            .map(|(i, s)| (i, s.parse::<i32>().unwrap()))
            .collect();

        for order in 0..v.len() {
            while v.front().unwrap().0 != order {
                v.rotate_left(1);
            }
            // println!("pulled {order} to front: {v:?}");
            let (index, value) = v.pop_front().unwrap();
            if value >= 0 {
                v.rotate_left(value as usize % v.len());
            } else {
                v.rotate_right((-value) as usize % v.len());
            }
            v.push_front((index, value));
            // println!("                   {v:?}");
        }

        while v.front().unwrap().1 != 0 {
            v.rotate_left(1);
        }

        let v1 = v.get(1000 % v.len()).unwrap().1;
        let v2 = v.get(2000 % v.len()).unwrap().1;
        let v3 = v.get(3000 % v.len()).unwrap().1;

        (v1 + v2 + v3).to_string()
    }

    fn part2(&self) -> String {
        let mut v: VecDeque<(usize, i64)> = self
            .input
            .lines()
            .enumerate()
            .map(|(i, s)| (i, 811589153 * s.parse::<i64>().unwrap()))
            .collect();

        for order in (0..v.len()).cycle().take(10 * v.len()) {
            while v.front().unwrap().0 != order {
                v.rotate_left(1);
            }
            // println!("pulled {order} to front: {v:?}");
            let (index, value) = v.pop_front().unwrap();
            if value >= 0 {
                v.rotate_left(value as usize % v.len());
            } else {
                v.rotate_right((-value) as usize % v.len());
            }
            v.push_front((index, value));
            // println!("                   {v:?}");
        }

        while v.front().unwrap().1 != 0 {
            v.rotate_left(1);
        }

        let v1 = v.get(1000 % v.len()).unwrap().1;
        let v2 = v.get(2000 % v.len()).unwrap().1;
        let v3 = v.get(3000 % v.len()).unwrap().1;

        (v1 + v2 + v3).to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::days::day20::Day20;
    use crate::days::Day;

    #[test]
    fn part1() {
        assert_eq!(
            Day20 {
                input: INPUT.to_string()
            }
            .part1(),
            "3"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Day20 {
                input: INPUT.to_string()
            }
            .part2(),
            "1623178306"
        );
    }

    const INPUT: &str = "1
2
-3
3
-2
0
4
";
}
