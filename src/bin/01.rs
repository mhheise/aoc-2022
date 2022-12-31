#![allow(unused_variables)]
use aoc::solve;
use color_eyre::Result;
use itertools::Itertools;

const DAY: u8 = 1;

fn parse(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|cal| cal.parse::<usize>().ok())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .collect::<Vec<usize>>()
}

fn p1(input: &str) -> Option<usize> {
    Some(parse(input)[0])
}

fn p2(input: &str) -> Option<usize> {
    Some(parse(input)[0..3].iter().sum())
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
