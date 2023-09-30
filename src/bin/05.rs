//! Day 5: Supply Stacks
#![feature(test)]

use aoc::prelude::*;

const DAY: u8 = 5;

fn parse(input: &str) -> Vec<&str> {
    input.split("\n\n").filter(|section| !section.is_empty()).collect()
}

fn number_of_stacks(stacks: &str) -> usize {
    (stacks.lines().last().unwrap().len() + 1) / 4
}

fn top_crates_by_stack(stacks: &[Vec<char>]) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).join("")
}

fn parse_crates(input: &str) -> Vec<Vec<char>> {
    let number_of_stacks = number_of_stacks(input);
    let mut crates = vec![Vec::new(); number_of_stacks];

    input.lines().rev().skip(1).for_each(|line| {
        line.chars().skip(1).step_by(4).enumerate().for_each(|(n, c)| {
            if c.is_ascii_uppercase() {
                crates[n].push(c);
            }
        });
    });

    crates
}

fn parse_steps(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .step_by(2)
                .filter_map(|num| num.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn p1(input: &str) -> Option<String> {
    let binding = parse(input);
    let mut crates = parse_crates(binding[0]);
    let steps = parse_steps(binding[1]);

    for (n, from, to) in steps {
        for _ in 0..n {
            let krate = crates[from - 1].pop().unwrap();
            crates[to - 1].push(krate);
        }
    }

    Some(top_crates_by_stack(&crates))
}

fn p2(input: &str) -> Option<String> {
    let binding = parse(input);
    let mut crates = parse_crates(binding[0]);
    let steps = parse_steps(binding[1]);

    for (n, from, to) in steps {
        let len = crates[from - 1].len() - n;
        let stack = crates[from - 1].drain(len..).collect::<Vec<_>>();
        crates[to - 1].extend(stack);
    }

    Some(top_crates_by_stack(&crates))
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use aoc::testing::*;

    use super::*;

    #[test]
    fn test_number_of_stacks() {
        let input = read_file("examples", DAY).unwrap();
        let (crates, _) = input.split_once("\n\n").unwrap();

        assert_eqp!(number_of_stacks(crates), 3);
    }

    #[test]
    fn test_top_crates_by_stack() {
        let input = read_file("examples", DAY).unwrap();
        let (crates, _) = input.split_once("\n\n").unwrap();
        let stacks = parse_crates(crates);

        assert_eqp!(top_crates_by_stack(&stacks), "NDP");
    }

    #[test]
    fn test_parse_crates() {
        let input = read_file("examples", DAY).unwrap();
        let (crates, _) = input.split_once("\n\n").unwrap();

        assert_eqp!(parse_crates(crates), vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
    }

    #[test]
    fn test_parse_steps() {
        let input = read_file("examples", DAY).unwrap();
        let (_, steps) = input.split_once("\n\n").unwrap();

        assert_eqp!(parse_steps(steps), vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]);
    }

    #[test]
    fn test_p1() {
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), Some("CMZ".to_string()));
    }

    #[test]
    fn test_p2() {
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), Some("MCD".to_string()));
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
