//! Day 10: Cathode-Ray Tube
#![feature(test)]

use aoc::prelude::*;

const DAY: u8 = 10;

fn run(input: &str) -> Vec<i32> {
    input
        .lines()
        .scan(1, |reg, tick| {
            Some(if let Some(val) = tick.strip_prefix("addx ") {
                let total = vec![*reg, *reg];
                *reg += val.parse::<i32>().unwrap();
                total
            } else {
                vec![*reg]
            })
        })
        .flatten()
        .collect()
}

fn p1(input: &str) -> Option<i32> {
    Some(
        run(input)
            .iter()
            .enumerate()
            .skip(19)
            .step_by(40)
            .map(|(cycle, x)| (cycle + 1) as i32 * x)
            .sum(),
    )
}

fn p2(input: &str) -> Option<String> {
    let screen = run(input)
        .iter()
        .enumerate()
        .filter_map(|(cycle, x)| {
            (x.abs_diff(cycle as i32 % 40) <= 1).then_some((cycle % 40, cycle / 40))
        })
        .collect::<Grid>();

    Some(crt_4(&format!("{screen:?}")))
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use aoc::testing::*;

    use super::*;

    #[test]
    fn test_p1() {
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), Some(13_140));
    }

    #[test]
    fn test_p2() {
        // Not a meaningful test
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), Some("????????".to_string()));
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
