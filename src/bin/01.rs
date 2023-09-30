//! Day 1: Calorie Counting
#![feature(test)]

use aoc::prelude::*;

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
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use aoc::testing::*;

    use super::*;

    #[test]
    fn test_p1() {
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), Some(24_000));
    }

    #[test]
    fn test_p2() {
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), Some(45_000));
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
