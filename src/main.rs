//! Advent of Code 2022
use std::{fs, process::Command};

use anyhow::Result;
use aoc::extract_solver_time;
use itertools::Itertools;
use tabled::{
    settings::{
        object::{Columns, Rows},
        Alignment, Modify, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Problem {
    day: String,
    p1: String,
    p2: String,
    time: String,
}

impl Problem {
    fn new(day: &str, p1: &str, p2: &str, time: &str) -> Self {
        Self {
            day: day.trim_start_matches('0').to_string(),
            p1: p1.to_string(),
            p2: p2.to_string(),
            time: time.to_string(),
        }
    }
}

fn main() -> Result<()> {
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .filter(|f| f.parse::<u8>().is_ok())
        .sorted()
        .collect::<Vec<_>>();

    let mut total_time = 0;
    let mut solutions: Vec<Problem> = Vec::new();

    for day in &days {
        let cmd = Command::new("cargo").args(["run", "--release", "--bin", day]).output()?;

        let output = String::from_utf8(cmd.stdout)?;
        let (p1, p2, time) = output.split_whitespace().collect_tuple().unwrap();

        solutions.push(Problem::new(day, p1, p2, time));
        total_time += extract_solver_time(&output)?;
    }

    let mut table = Table::new(solutions);
    table
        .with(Style::modern())
        .with(Modify::new(Rows::first()).with(Alignment::center()))
        .with(Modify::new(Columns::first()).with(Alignment::center()));

    println!("{table}");
    println!("\nTotal time: {:.2}ms", total_time / 1000);

    Ok(())
}
