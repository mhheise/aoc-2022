use anyhow::Result;
use itertools::Itertools;

fn day01_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn day01_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<usize>()
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/01.txt");

    let p1 = day01_part1(&input);
    let p2 = day01_part2(&input);

    println!("day01 -> part1: {p1} part2: {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const SAMPLE: &str = indoc! {r#"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "#};

    #[test]
    fn test_part1() {
        assert_eq!(day01_part1(&SAMPLE), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(day01_part2(&SAMPLE), 45000);
    }
}
