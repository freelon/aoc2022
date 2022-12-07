use crate::days::Day;

pub fn create(input: String) -> Box<dyn Day> {
    Box::new(Day02 { input })
}

struct Day02 {
    input: String,
}

impl Day for Day02 {
    fn part1(&self) -> String {
        let points: u32 = self
            .input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(' ').unwrap();

                game_points_part_1(a, b) + symbol_points(b)
            })
            .sum();
        format!("{points}")
    }

    fn part2(&self) -> String {
        let points: u32 = self
            .input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(' ').unwrap();

                let my_play = play_for_result(a, b);
                game_points_part_1(a, &my_play) + symbol_points(&my_play)
            })
            .sum();
        format!("{points}")
    }
}

fn play_for_result(a: &str, b: &str) -> String {
    let s = match a {
        "A" => match b {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => panic!(),
        },
        "B" => match b {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => panic!(),
        },
        "C" => match b {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => panic!(),
        },
        _ => panic!(),
    };
    s.into()
}

fn game_points_part_1(a: &str, b: &str) -> u32 {
    match a {
        "A" => match b {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => panic!(),
        },
        "B" => match b {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!(),
        },
        "C" => match b {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn symbol_points(x: &str) -> u32 {
    match x {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    }
}
