//! Day 3: Rucksack Reorganization
#![feature(test)]

use aoc::prelude::*;

const DAY: u8 = 3;

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn to_priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => u32::from(item - b'a' + 1),
        b'A'..=b'Z' => u32::from(item - b'A' + 27),
        _ => unreachable!(),
    }
}

fn p1(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .map(|sack| sack.split_at(sack.len() / 2))
            .map(|(left, right)| {
                left.iter().find(|item| right.contains(item)).copied().map(to_priority).unwrap()
            })
            .sum(),
    )
}

fn p2(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .chunks_exact(3)
            .map(|sack| {
                sack[0]
                    .iter()
                    .find(|badge| sack[1].contains(badge) && sack[2].contains(badge))
                    .unwrap()
            })
            .copied()
            .map(to_priority)
            .sum(),
    )
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use aoc::testing::*;

    use super::*;

    #[test]
    fn test_to_priority() {
        assert_eqp!(to_priority(b'a'), 1);
        assert_eqp!(to_priority(b'z'), 26);
        assert_eqp!(to_priority(b'A'), 27);
        assert_eqp!(to_priority(b'Z'), 52);
    }

    #[test]
    fn test_p1() {
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), Some(157));
    }

    #[test]
    fn test_p2() {
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), Some(70));
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
