#![allow(dead_code, unused_variables)]
use aoc::solve;
use color_eyre::Result;
use itertools::Itertools;

const DAY: u8 = 6;

fn run(input: &str, size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(size)
        .position(|window| window.iter().all_unique())
        .map(|pos| pos + size)
}

fn p1(input: &str) -> Option<usize> {
    run(input, 4)
}

fn p2(input: &str) -> Option<usize> {
    run(input, 14)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(solve!(&aoc::read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    use super::*;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_p1(input: &str, index: usize) {
        assert_eq!(p1(input), Some(index))
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_p2(input: &str, index: usize) {
        assert_eq!(p2(input), Some(index))
    }
}
