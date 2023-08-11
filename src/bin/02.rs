#![allow(dead_code, unused_variables)]
use aoc::solve;
use color_eyre::Result;

const DAY: u8 = 2;

fn parse(input: &str) -> Vec<(i16, i16)> {
    input
        .lines()
        .map(str::as_bytes)
        // convert from u8 to i16 to support wrap-around arithmetic
        .map(|c| ((c[0] - b'A') as i16, (c[2] - b'X') as i16))
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
    color_eyre::install()?;
    Ok(solve!(&aoc::read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_file("examples", DAY).unwrap()), Some(15));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_file("examples", DAY).unwrap()), Some(12));
    }
}
