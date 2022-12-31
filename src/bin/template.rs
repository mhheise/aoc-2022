#![allow(unused_variables)]
use aoc::solve;
use color_eyre::Result;

const DAY: u8 = 0;

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn p1(input: &str) -> Option<usize> {
    Some(parse(input)[0])
}

fn p2(input: &str) -> Option<usize> {
    Some(parse(input)[0])
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
        assert_eq!(p1(&aoc::read_file("examples", DAY).unwrap()), Some(0))
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_file("examples", DAY).unwrap()), Some(0))
    }
}
