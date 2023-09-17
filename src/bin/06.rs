//! Day 6: Tuning Trouble
use aoc::prelude::*;

const DAY: u8 = 6;

fn run<const S: usize>(input: &str) -> Option<usize> {
    input.as_bytes().windows(S).position(|window| window.iter().all_unique()).map(|pos| pos + S)
}

fn p1(input: &str) -> Option<usize> {
    run::<4>(input)
}

fn p2(input: &str) -> Option<usize> {
    run::<14>(input)
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_p1(input: &str, index: usize) {
        assert_eq!(p1(input), Some(index));
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_p2(input: &str, index: usize) {
        assert_eq!(p2(input), Some(index));
    }
}
