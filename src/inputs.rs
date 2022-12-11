use std::fs::{File, read_to_string};
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

use reqwest::blocking::Client;

const YEAR: u32 = 2022;
const SECRET_PATH: &str = "./session-secret.txt";

pub fn ensure_input_exists(day: u8) {
    let path = format!("input/day{:02}.txt", day);
    let path = Path::new(&path);
    if !Path::exists(path) {
        let input = load_external(day);

        let mut file = File::create(path).unwrap();
        write!(file, "{}", input).unwrap();
    }
}

fn load_external(day: u8) -> String {
    let session_id = read_to_string(SECRET_PATH).unwrap_or_else(|_| {
        eprintln!(
            "Expected a file containing the session secret at '{}'",
            SECRET_PATH
        );
        exit(-1)
    });

    let client = Client::new();
    let res = client
        .get(&format!(
            "https://adventofcode.com/{}/day/{}/input",
            YEAR, day
        ))
        .header("Cookie", format!("session={}", session_id.trim()))
        .header(
            "User-Agent",
            "github.com/freelon/aoc2022 freelon at gmx dot net",
        )
        .send()
        .expect("Couldn't get input from server");
    if !res.status().is_success() {
        panic!("Bad response: {} - {:?}", res.status(), res.text());
    }

    res.text().expect("couldn't read body")
}
