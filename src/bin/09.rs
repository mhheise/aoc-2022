//! Day 9: Rope Bridge
use aoc::prelude::*;

const DAY: u8 = 9;

fn parse(input: &str) -> Vec<u8> {
    input
        .lines()
        .filter_map(|line| {
            let (dir, n) = line.split_once(' ')?;
            Some((dir.as_bytes()[0], n.parse().ok()?))
        })
        .flat_map(|(d, n)| vec![d; n])
        .collect()
}

fn run<const S: usize>(steps: Vec<u8>) -> Option<usize> {
    let mut rope = vec![(0, 0); S];
    let mut visited = FxHashSet::default();

    for step in steps {
        match step {
            b'U' => rope[0].1 += 1,
            b'R' => rope[0].0 += 1,
            b'D' => rope[0].1 -= 1,
            b'L' => rope[0].0 -= 1,
            _ => unreachable!(),
        }

        for (head, tail) in (0..S).tuple_windows() {
            let dx: i16 = rope[head].0 - rope[tail].0;
            let dy: i16 = rope[head].1 - rope[tail].1;

            // If the head is more than one step away from the tail, move the tail
            // one step in that direction so that it is adjacent to the head
            if dx.abs() > 1 || dy.abs() > 1 {
                rope[tail].0 += dx.signum();
                rope[tail].1 += dy.signum();
            }

            if rope[tail] == rope[S - 1] {
                visited.insert(rope[tail]);
            }
        }
    }

    Some(visited.len())
}

fn p1(input: &str) -> Option<usize> {
    run::<2>(parse(input))
}

fn p2(input: &str) -> Option<usize> {
    run::<10>(parse(input))
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        // Uses the longer example from part 2
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), Some(88));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), Some(36));
    }
}
