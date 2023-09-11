//! Day 8: Treetop Tree House
use aoc::prelude::*;

const DAY: u8 = 8;

fn parse(input: &str) -> Matrix<&u8> {
    Matrix::from_rows(input.lines().map(str::as_bytes)).unwrap()
}

fn is_tree_visible(coord: (usize, usize), matrix: &Matrix<&u8>) -> bool {
    let (row, col) = coord;
    let height = matrix[(row, col)];

    DIRECTIONS_4
        .iter()
        // Count a tree visible in more than one direction only once
        .any(|dir| {
            // Assert that all other trees in this direction between the current tree
            // and the edge of the forest are shorter than the current tree
            matrix
                .in_direction((row, col), *dir)
                .all(|(x, y)| matrix[(x, y)] < height)
        })
}

fn calc_scenic_score(coord: (usize, usize), matrix: &Matrix<&u8>) -> usize {
    let (row, col) = coord;
    let height = matrix[(row, col)];

    DIRECTIONS_4
        .iter()
        .map(|dir| {
            // Number of trees between the current tree and the edge of the forest
            let len = matrix.in_direction((row, col), *dir).count();
            // Stop at the first tree that is the same height or taller. If no trees are
            // taller, stop at the edge of the forest and use that distance instead.
            matrix
                .in_direction((row, col), *dir)
                .position(|h| matrix[h] >= height)
                .map_or_else(|| len, |p| p + 1)
        })
        .product()
}

fn p1(input: &str) -> Option<usize> {
    let matrix = parse(input);

    Some(matrix.keys().filter(|&(row, col)| is_tree_visible((row, col), &matrix)).count())
}

fn p2(input: &str) -> Option<usize> {
    let matrix = parse(input);

    Some(matrix.keys().map(|(row, col)| calc_scenic_score((row, col), &matrix)).max().unwrap())
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case((1, 1), true)]
    #[test_case((1, 2), true)]
    #[test_case((1, 3), false)]
    #[test_case((2, 1), true)]
    #[test_case((2, 2), false)]
    #[test_case((2, 3), true)]
    #[test_case((3, 1), false)]
    #[test_case((3, 2), true)]
    #[test_case((3, 3), false)]
    fn test_is_tree_visible(coord: (usize, usize), result: bool) {
        let input = &read_file("examples", DAY).unwrap();
        let matrix = parse(input);
        assert_eq!(is_tree_visible(coord, &matrix), result);
    }

    #[test_case((1, 1), 1)]
    #[test_case((1, 2), 4)]
    #[test_case((1, 3), 1)]
    #[test_case((2, 1), 6)]
    #[test_case((2, 2), 1)]
    #[test_case((2, 3), 2)]
    #[test_case((3, 1), 1)]
    #[test_case((3, 2), 8)]
    #[test_case((3, 3), 3)]
    fn test_calc_scenic_score(coord: (usize, usize), result: usize) {
        let input = &read_file("examples", DAY).unwrap();
        let matrix = parse(input);
        assert_eq!(calc_scenic_score(coord, &matrix), result);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), Some(21));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), Some(8));
    }
}
