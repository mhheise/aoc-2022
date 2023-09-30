//! Day 0: Template
#![allow(dead_code, unused_variables)]
#![feature(test)]

use aoc::prelude::*;

const DAY: u8 = 0;

fn parse(input: &str) -> Vec<usize> {
    input.lines().filter_map(|line| line.parse::<usize>().ok()).collect()
}

const fn p1(input: &str) -> Option<usize> {
    None
}

const fn p2(input: &str) -> Option<usize> {
    None
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
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), None);
    }

    #[test]
    fn test_p2() {
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), None);
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
