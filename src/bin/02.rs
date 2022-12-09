use anyhow::Result;

fn day02_part1(input: &str) -> usize {
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

    input
        .lines()
        .map(|game| game.split(' ').collect::<Vec<&str>>())
        .map(calc)
        .sum()
}

fn day02_part2(input: &str) -> usize {
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

    input
        .lines()
        .map(|game| game.split(' ').collect::<Vec<&str>>())
        .map(calc)
        .sum()
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/02.txt");

    let p1 = day02_part1(input);
    let p2 = day02_part2(input);

    println!("day02 -> part1: {p1} part2: {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {r#"
        A Y
        B X
        C Z
    "#};

    #[test]
    fn test_part1() {
        assert_eq!(day02_part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(day02_part2(SAMPLE), 12);
    }
}
