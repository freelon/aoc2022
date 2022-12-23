use std::collections::HashSet;

use itertools::Itertools;

use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day20 { input })
}

struct Day20 {
    input: String,
}

impl Day for Day20 {
    fn part1(&self) -> String {
        let mut v = self
            .input
            .lines()
            .map(|line| (false, line.parse::<i32>().unwrap()))
            .collect_vec();

        let set: HashSet<i32> = v.iter().map(|x| x.1).collect();

        assert_eq!(v.len(), set.len());

        let l = v.len();

        let mut moved_count = 0;
        let mut index = 0;
        while moved_count < l {
            println!("{:?}", v);
            let (_, i) = v.remove(index);
            let mut new_index = (index as i32 + i).rem_euclid(l as i32) as usize;
            if new_index < index {
                new_index += 1;
                index = (index + 1) % l;
            }
            v.insert(new_index, (true, i));
            moved_count += 1;
            while v[index].0 == false {
                index = (index + 1) % l;
            }
        }

        println!("{:?}", v);

        let index_0 = v.iter().find_position(|(_, v)| *v == 0).unwrap().0;
        let i1 = (index_0 + 1000) % l;
        let i2 = (index_0 + 2000) % l;
        let i3 = (index_0 + 3000) % l;

        dbg!(index_0);
        dbg!(i1);
        dbg!(i2);
        dbg!(i3);

        let v1 = v[i1].1;
        let v2 = v[i2].1;
        let v3 = v[i3].1;

        dbg!(v1);
        dbg!(v2);
        dbg!(v3);

        (v1 + v2 + v3).to_string()
    }

    fn part2(&self) -> String {
        format!("")
    }
}

#[cfg(test)]
mod test {
    use crate::days::Day;
    use crate::days::day20::Day20;

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

    const INPUT: &str = "1
2
-3
3
-2
0
4
";
}
