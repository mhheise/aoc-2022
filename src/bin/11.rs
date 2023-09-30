//! Day 11: Monkey in the Middle
#![feature(test)]

use aoc::prelude::*;

const DAY: u8 = 11;

#[derive(Debug, Display, FromStr)]
enum Operation {
    #[display(r"old * old")]
    Square,
    #[display(r"old + {0}")]
    Add(u64),
    #[display(r"old * {0}")]
    Multiply(u64),
}

#[derive(Debug, Display, FromStr)]
#[display(r"Monkey {id}:|  Starting items: {worry}|  Operation: new = {operation}|  Test: divisible by {divisor}|    If true: throw to monkey {true_monkey}|    If false: throw to monkey {false_monkey}")]
struct Monkey {
    id: u8,
    worry: String,
    operation: Operation,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
    #[from_str(default, ignore)]
    items: VecDeque<u64>,
    #[from_str(default, ignore)]
    inspected: u64,
}

impl Monkey {
    fn inspect(&self, item: u64, manage: impl Fn(u64) -> u64) -> u64 {
        debug!("  Monkey inspects an item with a worry level of {}.", item);
        manage(match self.operation {
            Operation::Square => {
                debug!("    Worry level is multiplied by itself to {}.", item * item);
                item * item
            }
            Operation::Add(n) => {
                debug!("    Worry level increases by {} to {}.", n, n + item);
                n + item
            }
            Operation::Multiply(n) => {
                debug!("    Worry level is multiplied by {n} to {}.", n * item);
                n * item
            }
        })
    }

    fn test(&self, new: u64) -> usize {
        if new % self.divisor == 0 {
            debug!("    Current worry level is divisible by {}.", self.divisor);
            debug!("    Item with worry level {} is thrown to monkey {}.", new, self.true_monkey);
            self.true_monkey
        } else {
            debug!("    Current worry level is not divisible by {}.", self.divisor);
            debug!("    Item with worry level {} is thrown to monkey {}.", new, self.false_monkey);
            self.false_monkey
        }
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| block.trim().replace('\n', "|"))
        .map(|block| {
            let mut monkey = block.parse::<Monkey>().unwrap();
            // Workaround for `parse-display` not supporting vectors directly
            monkey.items = monkey.worry.split(", ").filter_map(|item| item.parse().ok()).collect();
            monkey
        })
        .collect()
}

fn simulate(monkeys: &mut Vec<Monkey>, rounds: u32, manage: impl Fn(u64) -> u64) {
    for round in 1..=rounds {
        debug!("Round {}\n", round);
        for i in 0..monkeys.len() {
            debug!("Monkey {}:", i);
            while let Some(item) = monkeys[i].items.pop_front() {
                let monkey = &monkeys[i];
                let new = monkey.inspect(item, &manage);
                let target = monkey.test(new);

                monkeys[i].inspected += 1;
                monkeys[target].items.push_back(new);
            }
        }

        debug!("");
        debug!("After round {round}, the monkeys are holding items with these worry levels:");
        for (i, monkey) in monkeys.iter().enumerate() {
            debug!("  Monkey {}: {}", i, monkey.items.iter().join(", "));
            if i == round as usize {
                debug!("Monkey {} inspected items {} times.", i, monkey.inspected);
            }
        }
        debug!("");
    }
}

fn calc_monkey_business(monkeys: &[Monkey]) -> u64 {
    monkeys.iter().map(|i| i.inspected).sorted().rev().take(2).product()
}

fn p1(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    simulate(&mut monkeys, 20, |w| w / 3);

    Some(calc_monkey_business(&monkeys))
}

fn p2(input: &str) -> Option<u64> {
    let mut monkeys = parse(input);
    let modulo = monkeys.iter().map(|m| m.divisor).product::<u64>();
    simulate(&mut monkeys, 10_000, |w| w % modulo);

    Some(calc_monkey_business(&monkeys))
}

fn main() -> Result<()> {
    init_tracing();
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use aoc::testing::*;

    use super::*;

    #[test]
    fn test_p1() {
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), Some(10_605));
    }

    #[test]
    fn test_p2() {
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), Some(2_713_310_158));
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
