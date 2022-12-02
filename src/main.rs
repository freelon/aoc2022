use std::process::exit;

use chrono::{Datelike, DateTime, Local};
use clap::Parser;

use crate::days::{ALL, run};

mod days;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(group = "days-to-run")]
    #[arg(long)]
    day: Option<u8>,
    #[clap(group = "days-to-run")]
    #[arg(long)]
    run_all: bool,
}

fn main() {
    let cli = Cli::parse();

    let days: Vec<u8> = if let Some(day) = cli.day {
        vec![day]
    } else if cli.run_all {
        let mut all_days: Vec<u8> = ALL.iter().map(|(d, _)| *d).collect();
        all_days.sort();
        all_days
    } else {
        let local: DateTime<Local> = Local::now();
        if local.month() != 12 {
            println!("Running the current day only works in december (try --help)");
            exit(-1)
        }
        let day = local.day();
        if day > 25 {
            println!("AoC is done. If you want to run a specific day, you have to say so (try --help)");
            exit(-1)
        }
        vec![day as u8]
    };
    run(days);
}
