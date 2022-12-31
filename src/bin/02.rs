#![allow(unused_variables)]
use aoc::solve;
use color_eyre::Result;

const DAY: u8 = 2;

fn p1(input: &str) -> Option<usize> {
    // X, Y, Z -> 1, 2, 3
    // win, draw, loss -> 6, 3, 0
    let calc = |round: Vec<&str>| match round[..] {
        ["A", "X"] => 1 + 3,
        ["A", "Y"] => 2 + 6,
        ["A", "Z"] => 3,
        ["B", "X"] => 1,
        ["B", "Y"] => 2 + 3,
        ["B", "Z"] => 3 + 6,
        ["C", "X"] => 1 + 6,
        ["C", "Y"] => 2,
        ["C", "Z"] => 3 + 3,
        _ => unreachable!(),
    };

    Some(
        input
            .lines()
            .map(|game| game.split(' ').collect::<Vec<&str>>())
            .map(calc)
            .sum(),
    )
}

pub fn p2(input: &str) -> Option<usize> {
    // X, Y, Z now means lose, draw, win
    let calc = |round: Vec<&str>| match round[..] {
        ["A", "X"] => 3,     // scissors
        ["A", "Y"] => 1 + 3, // rock
        ["A", "Z"] => 2 + 6, // paper
        ["B", "X"] => 1,     // rock
        ["B", "Y"] => 2 + 3, // paper
        ["B", "Z"] => 3 + 6, // scissors
        ["C", "X"] => 2,     // paper
        ["C", "Y"] => 3 + 3, // scissors
        ["C", "Z"] => 1 + 6, // rock
        _ => unreachable!(),
    };

    Some(
        input
            .lines()
            .map(|game| game.split(' ').collect::<Vec<&str>>())
            .map(calc)
            .sum(),
    )
}

fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(solve!(&aoc::read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&aoc::read_file("examples", DAY).unwrap()), Some(15));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&aoc::read_file("examples", DAY).unwrap()), Some(12));
    }
}
