//! Day 4: Camp Cleanup
#![feature(test)]

use aoc::prelude::*;

const DAY: u8 = 4;

fn parse(input: &str) -> Vec<(u8, u8, u8, u8)> {
    input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once(',')?;
            let (a, b) = left.split_once('-')?;
            let (c, d) = right.split_once('-')?;
            Some((a.parse().ok()?, b.parse().ok()?, c.parse().ok()?, d.parse().ok()?))
        })
        .collect()
}

fn p1(input: &str) -> Option<usize> {
    Some(
        parse(input).iter().filter(|(a, b, c, d)| (a <= c && b >= d) || (a >= c && b <= d)).count(),
    )
}

fn p2(input: &str) -> Option<usize> {
    Some(parse(input).iter().filter(|(a, b, c, d)| (a <= d && c <= b)).count())
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use aoc::testing::*;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eqp!(
            parse(&read_file("examples", DAY).unwrap()),
            vec![
                (2, 4, 6, 8),
                (2, 3, 4, 5),
                (5, 7, 7, 9),
                (2, 8, 3, 7),
                (6, 6, 4, 6),
                (2, 6, 4, 8)
            ]
        );
    }

    #[test]
    fn test_p1() {
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), Some(2));
    }

    #[test]
    fn test_p2() {
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), Some(4));
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
