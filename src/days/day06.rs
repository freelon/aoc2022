use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day06 { input })
}

struct Day06 {
    input: String,
}

impl Day for Day06 {
    fn part1(&self) -> String {
        format!("")
    }

    fn part2(&self) -> String {
        format!("")
    }
}
