use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(DayXX { input })
}

struct DayXX {
    input: String,
}

impl Day for DayXX {
    fn part1(&self) -> String {
        format!("")
    }

    fn part2(&self) -> String {
        format!("")
    }
}
