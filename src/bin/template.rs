#![allow(dead_code, unused_variables)]
use aoc::prelude::*;

const DAY: u8 = 0;

fn p1(input: &str) -> Option<u32> {
    None
}

fn p2(input: &str) -> Option<u32> {
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
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), None)
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), None)
    }
}
