use std::fs::read_to_string;
use std::process::exit;

mod day01;

pub fn run(days_to_run: Vec<u8>) {
    let tasks: Vec<(u8, Box<dyn Day>)> = days_to_run
        .into_iter()
        .map(|day| {
            let fun = match day {
                0 => panic!("0 day is an exploit, not the day of a month!"),
                1 => day01::create,
                _ => {
                    eprintln!("Error: day {day} is not yet implemented");
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
