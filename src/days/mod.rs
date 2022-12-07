use std::fs::read_to_string;
use std::process::exit;
use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

type CreateFn = fn(String) -> Box<dyn Day>;

pub const ALL: [(u8, CreateFn); 7] = [
    (1, day01::create),
    (2, day02::create),
    (3, day03::create),
    (4, day04::create),
    (5, day05::create),
    (6, day06::create),
    (7, day07::create),
];

pub fn run(days_to_run: Vec<u8>, collect_timing: bool) {
    let tasks: Vec<(u8, Box<dyn Day>)> = days_to_run
        .into_iter()
        .map(|day| {
            let fun = match day {
                0 => {
                    eprintln!("0 day is an exploit, not the day of a month!");
                    exit(-1)
                }
                1..=25 => {
                    ALL.iter()
                        .find(|(d, _)| *d == day)
                        .unwrap_or_else(|| {
                            eprintln!("Error: day {day} is not yet implemented");
                            exit(-1)
                        })
                        .1
                }
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
        let start = Instant::now();
        let solution = task.part1();
        let after1 = Instant::now();
        println!("Day {day} part 1 solution: {solution}");
        let solution = task.part2();
        let after2 = Instant::now();
        println!("Day {day} part 2 solution: {solution}");

        if collect_timing {
            println!(
                "Day {day} part 1 took {:?}, part 2 took {:?}",
                after1.duration_since(start),
                after2.duration_since(after1)
            )
        }
    }
}

pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
