#![allow(dead_code, unused_variables)]
use aoc::solve;
use color_eyre::Result;
use itertools::Itertools;

const DAY: u8 = 1;

fn parse(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().filter_map(|cal| cal.parse::<u32>().ok()).sum())
        .sorted()
        .rev()
        .collect()
}

fn p1(input: &str) -> Option<u32> {
    Some(parse(input)[0])
}

fn p2(input: &str) -> Option<u32> {
    Some(parse(input).iter().take(3).sum())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(solve!(&aoc::read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_file("examples", DAY).unwrap()), Some(24000))
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_file("examples", DAY).unwrap()), Some(45000))
    }
}
