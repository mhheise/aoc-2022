//! Day 6: Tuning Trouble
#![feature(test)]

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
    use aoc::testing::*;

    use super::*;

    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_p1(input: &str, index: usize) {
        assert_eqp!(p1(input), Some(index));
    }

    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_p2(input: &str, index: usize) {
        assert_eqp!(p2(input), Some(index));
    }

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = read_file("inputs", DAY).unwrap();
        b.iter(|| p1(&input));
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = read_file("inputs", DAY).unwrap();
        b.iter(|| p2(&input));
    }
}
