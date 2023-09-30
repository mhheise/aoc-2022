//! Day 12: Hill Climbing Algorithm
#![feature(test)]

use aoc::prelude::*;

const DAY: u8 = 12;

fn parse(input: &str) -> (Matrix<u8>, (usize, usize), (usize, usize)) {
    let mut matrix = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let start = matrix.items().find(|(_, &p)| p == b'S').unwrap().0;
    let end = matrix.items().find(|(_, &p)| p == b'E').unwrap().0;
    // ASCII uppercase letters are lower-valued than lowercase letters
    matrix[start] = b'a';
    matrix[end] = b'z';
    (matrix, start, end)
}

fn p1(input: &str) -> Option<usize> {
    let (ref matrix, start, end) = parse(input);

    // Use breadth-first search to find the shortest path from `a` to the end
    let search = bfs(
        &start,
        |&p| matrix.neighbours(p, false).filter(move |&q| matrix[q] <= matrix[p] + 1),
        |&e| e == end,
    );

    Some(search?.len() - 1)
}

fn p2(input: &str) -> Option<usize> {
    let (ref matrix, _, end) = parse(input);

    // Reverse the logic to find the shortest path from the end to any `a` tile
    let search = bfs(
        &end,
        |&p| matrix.neighbours(p, false).filter(move |&q| matrix[p] <= matrix[q] + 1),
        |&e| matrix[e] == b'a',
    );

    Some(search?.len() - 1)
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
        assert_eqp!(p1(&read_file("examples", DAY).unwrap()), Some(31));
    }

    #[test]
    fn test_p2() {
        assert_eqp!(p2(&read_file("examples", DAY).unwrap()), Some(29));
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
