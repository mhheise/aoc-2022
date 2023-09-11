//! Day 2: Rock Paper Scissors
use aoc::prelude::*;

const DAY: u8 = 2;

fn parse(input: &str) -> Vec<(i16, i16)> {
    input
        .lines()
        .map(str::as_bytes)
        // convert from u8 to i16 to support wrap-around arithmetic
        .map(|c| (i16::from(c[0] - b'A'), i16::from(c[2] - b'X')))
        .collect()
}

fn p1(input: &str) -> Option<i16> {
    Some(
        parse(input)
            .iter()
            // outcome = ours - theirs + 1 (modulo 3)
            .map(|(theirs, ours)| {
                let outcome = (ours - theirs + 1).rem_euclid(3);
                let shape = ours + 1;
                3 * outcome + shape
            })
            .sum::<i16>(),
    )
}

fn p2(input: &str) -> Option<i16> {
    Some(
        parse(input)
            .iter()
            // ours = theirs + outcome - 1 (modulo 3)
            .map(|(theirs, outcome)| {
                let ours = (theirs + outcome - 1).rem_euclid(3);
                let shape = ours + 1;
                3 * outcome + shape
            })
            .sum::<i16>(),
    )
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), Some(15));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), Some(12));
    }
}
