#![allow(dead_code, unused_variables)]
use aoc::solve;
use color_eyre::Result;

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
    color_eyre::install()?;
    Ok(solve!(&aoc::read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(&aoc::read_file("examples", DAY).unwrap()),
            vec![
                (2, 4, 6, 8),
                (2, 3, 4, 5),
                (5, 7, 7, 9),
                (2, 8, 3, 7),
                (6, 6, 4, 6),
                (2, 6, 4, 8)
            ]
        )
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_file("examples", DAY).unwrap()), Some(2))
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_file("examples", DAY).unwrap()), Some(4))
    }
}
