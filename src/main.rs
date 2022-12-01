use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day01.txt").unwrap();
    let m: Option<i32> = input.split("\n\n").map(|elve| elve.lines().map(|line| line.parse::<i32>().unwrap()).sum()).max();
    println!("{:?}", m);
}
