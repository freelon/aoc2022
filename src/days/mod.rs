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
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

type CreateFn = fn(String) -> Box<dyn Day>;

pub const ALL: &[(u8, CreateFn)] = &[
    (1, day01::create),
    (2, day02::create),
    (3, day03::create),
    (4, day04::create),
    (5, day05::create),
    (6, day06::create),
    (7, day07::create),
    (8, day08::create),
    (9, day09::create),
    (10, day10::create),
    (11, day11::create),
    (12, day12::create),
    (13, day13::create),
    (14, day14::create),
    (15, day15::create),
    (16, day16::create),
    (17, day17::create),
    (18, day18::create),
    (19, day19::create),
    (20, day20::create),
    (21, day21::create),
    (22, day22::create),
    (23, day23::create),
    (24, day24::create),
    (25, day25::create),
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

            let path = format!("input/day{:02}.txt", day);
            let input_of_day = read_to_string(&path).unwrap_or_else(|_| {
                println!("Missing input file '{}'", path);
                exit(-1)
            });
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
