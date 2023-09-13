//! Day 7: No Space Left On Device
use aoc::prelude::*;

const DAY: u8 = 7;

fn parse(input: &str) -> FxHashMap<PathBuf, u32> {
    let mut fs: FxHashMap<PathBuf, u32> = FxHashMap::default();
    let mut candidates: Vec<PathBuf> = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        match parts[..] {
            ["$", "cd", ".."] => {
                candidates.pop();
            }
            ["$", "cd", path] => {
                candidates.push(path.into());
            }
            [size, _name] => {
                let size = size.parse::<u32>().unwrap();
                for idx in 0..candidates.len() {
                    let path = PathBuf::from_iter(&candidates[..=idx]);
                    *fs.entry(path).or_insert(0) += size;
                }
            }
            _ => unreachable!(),
        }
    }

    fs
}

fn p1(input: &str) -> Option<u32> {
    let fs = parse(input);

    Some(fs.into_values().filter(|size| *size <= 100_000).sum::<u32>())
}

fn p2(input: &str) -> Option<u32> {
    let fs = parse(input);
    let root = fs.get(&PathBuf::from("/"))?;
    let available = 70_000_000 - root;

    Some(fs.into_values().filter(|size| *size + available >= 30_000_000).min().unwrap())
}

fn main() -> Result<()> {
    Ok(solve!(&read_file("inputs", DAY)?, p1, p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut fs: FxHashMap<PathBuf, u32> = FxHashMap::default();
        fs.insert("/d".into(), 24_933_642);
        fs.insert("/a/e".into(), 584);
        fs.insert("/a".into(), 94_853);
        fs.insert("/".into(), 48_381_165);

        assert_eq!(parse(&read_file("examples", DAY).unwrap()), fs);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&read_file("examples", DAY).unwrap()), Some(95_437));
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&read_file("examples", DAY).unwrap()), Some(24_933_642));
    }
}
