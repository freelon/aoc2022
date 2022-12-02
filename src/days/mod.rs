use std::fs::read_to_string;
use std::process::exit;

mod day01;
mod day02;

pub const ALL: [(u8, fn(String) -> Box<dyn Day>); 2] = [(1, day01::create), (2, day02::create)];

pub fn run(days_to_run: Vec<u8>) {
    let tasks: Vec<(u8, Box<dyn Day>)> = days_to_run
        .into_iter()
        .map(|day| {
            let fun = match day {
                0 => {
                    eprintln!("0 day is an exploit, not the day of a month!");
                    exit(-1)
                }
                1..=25 => ALL.iter().find(|(d, _)| *d == day).unwrap_or_else(|| {
                    eprintln!("Error: day {day} is not yet implemented");
                    exit(-1)
                }).1,
                _ => {
                    eprintln!("Day {day}? You already missed christmas! :o");
                    exit(-1)
                }
            };
            let input_of_day = read_to_string(format!("input/day{:02}.txt", day)).unwrap();
            let task = (fun)(input_of_day);
            (day, task)
        })
        .collect();

    for (day, task) in tasks {
        let solution = task.part1();
        println!("Day {day} part 1 solution: {solution}");
        let solution = task.part2();
        println!("Day {day} part 2 solution: {solution}");
    }
}

pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
