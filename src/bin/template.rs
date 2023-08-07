#![allow(dead_code, unused_variables)]
use aoc::solve;
use color_eyre::Result;

const DAY: u8 = 0;

fn p1(input: &str) -> Option<u32> {
    None
}

fn p2(input: &str) -> Option<u32> {
    None
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(solve!(&aoc::read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_file("examples", DAY).unwrap()), None)
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_file("examples", DAY).unwrap()), None)
    }
}
