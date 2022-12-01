use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day01.txt").unwrap();
    let mut snacks: Vec<i32> = input.split("\n\n").map(|elve| elve.lines().map(|line| line.parse::<i32>().unwrap()).sum()).collect();
    snacks.sort();
    let x: i32 = snacks.iter().rev().take(3).sum();
    println!("{}", x);
}
