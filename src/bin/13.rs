//! Day 13: Distress Signal
use aoc::prelude::*;

const DAY: u8 = 13;

#[derive(Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{n}"),
            Self::List(list) => f.debug_list().entries(list).finish(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<[Packet; 2]> {
    input
        .split("\n\n")
        .filter_map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            let left = from_str(left).ok()?;
            let right = from_str(right).ok()?;
            Some([left, right])
        })
        .collect()
}

fn p1(input: &str) -> Option<usize> {
    let groups = parse(input);

    Some(groups.iter().positions(|[left, right]| left < right).map(|i| i + 1).sum())
}

fn p2(input: &str) -> Option<usize> {
    let groups = parse(input);
    let two = &Packet::List(vec![Packet::Num(2)]);
    let six = &Packet::List(vec![Packet::Num(6)]);

    Some(
        groups
            .iter()
            .flatten()
            .chain([two, six])
            .sorted()
            .positions(|packet| packet == two || packet == six)
            .map(|i| i + 1)
            .product(),
    )
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    // `Ordering::Less` means the packets are in the right order;
    // `Ordering::Greater` means the packets are _not_ in the right order.
    #[test_case(("[1,1,3,1,1]", "[1,1,5,1,1]"), Ordering::Less)]
    #[test_case(("[[1],[2,3,4]]", "[[1],4]"), Ordering::Less)]
    #[test_case(("[9]", "[[8,7,6]]"), Ordering::Greater)]
    #[test_case(("[[4,4],4,4]", "[[4,4],4,4,4]"), Ordering::Less)]
    #[test_case(("[7,7,7,7]", "[7,7,7]"), Ordering::Greater)]
    #[test_case(("[]", "[3]"), Ordering::Less)]
    #[test_case(("[[[]]]", "[[]]"), Ordering::Greater)]
    #[test_case(("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"), Ordering::Greater)]
    fn test_cmp((l, r): (&str, &str), ordered: Ordering) {
        let left = from_str::<Packet>(l).unwrap();
        let right = from_str::<Packet>(r).unwrap();
        assert_eq!(left.partial_cmp(&right).unwrap(), ordered);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), Some(13));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), Some(140));
    }
}
