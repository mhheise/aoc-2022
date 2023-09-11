//! Day 3: Rucksack Reorganization
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
            .chunks(3)
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
    use super::*;

    #[test]
    fn test_to_priority() {
        assert_eq!(to_priority(b'a'), 1);
        assert_eq!(to_priority(b'z'), 26);
        assert_eq!(to_priority(b'A'), 27);
        assert_eq!(to_priority(b'Z'), 52);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), Some(157));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), Some(70));
    }
}
