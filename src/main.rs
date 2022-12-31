use aoc::extract_solver_time;
use color_eyre::Result;
use itertools::Itertools;
use std::{fs, process::Command};

fn main() -> Result<()> {
    color_eyre::install()?;

    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();

    let mut total_time = 0;

    for day in &days {
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", day])
            .output()?;

        let output = String::from_utf8(cmd.stdout)?;

        println!("Day {}:\n{}", day.trim_start_matches('0'), output);

        total_time += extract_solver_time(&output)?;
    }

    println!("Total time: {:.2}ms", total_time / 1000);

    Ok(())
}
