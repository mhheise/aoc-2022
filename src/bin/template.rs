#![allow(dead_code, unused_variables, clippy::missing_const_for_fn)]

//! Day 0: Template
use aoc::prelude::*;

const DAY: u8 = 0;

fn parse(input: &str) -> Vec<usize> {
    input.lines().filter_map(|line| line.parse::<usize>().ok()).collect()
}

fn p1(input: &str) -> Option<usize> {
    None
}

fn p2(input: &str) -> Option<usize> {
    None
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), None);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), None);
    }
}
