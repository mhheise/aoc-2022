use itertools::Itertools;
use rustc_hash::FxHashMap;
use tracing_subscriber::{fmt, EnvFilter};

/// Initializes tracing with the default environment filter and compact output format.
///
/// This is useful for conditionally rendering debug log messages based on the `RUST_LOG`
/// environment variable.
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(
            fmt::format()
                .without_time()
                .with_level(false)
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .compact(),
        )
        .init();
}

/// Returns a series of uppercase letters corresponding to "lit" characters on a screen.
///
/// Assumes the following:
///   - The characters on the grid are represented using `#` and `.`, corresponding to "lit" and
///     "dim" characters, respectively
///   - Each character is 4x6 pixels
///   - Each character is separated by a single column of "dim" pixels.
///
/// If a letter is not recognized, a question mark is returned.
///
/// # Examples
///
/// ```rust
/// use aoc::prelude::*;
/// use indoc::indoc;
///
/// let screen = indoc! {r"
/// #   ###...###
/// #   #..#.#...
/// #   #..#.#...
/// #   ###...##.
/// #   #.#.....#
/// #   #..#.###.
/// "};
///
/// assert_eq!(crt_4(&screen), "RS");
/// ```
///
/// # Panics
///
/// This function will panic if the screen is empty.
#[must_use]
pub fn crt_4(screen: &str) -> String {
    const SPAN_4: usize = 4;

    let cols = screen.lines().last().unwrap().len() + 1;
    let mut letters = Vec::new();

    for i in 1..=(cols / (SPAN_4 + 1)) {
        let letter = &screen
            .lines()
            .map(|line| line.chars().skip((i - 1) * (SPAN_4 + 1)).take(SPAN_4).collect::<String>())
            .join("\n");

        letters.push(ocr_4(letter));
    }

    letters.iter().collect()
}

/// Returns the character representing an uppercase letter in 4x6 pixel format.
///
/// If the letter is not recognized, a question mark is returned.
///
/// # Examples
///
/// ```rust
/// use aoc::prelude::*;
///
/// assert_eq!(ocr_4(".##.\n#..#\n#..#\n####\n#..#\n#..#"), 'A');
/// assert_eq!(ocr_4("###.\n#..#\n###.\n#..#\n#..#\n###."), 'B');
/// assert_eq!(ocr_4(".##.\n#..#\n#...\n#...\n#..#\n.##."), 'C');
/// ```
#[must_use]
pub fn ocr_4(letter: &str) -> char {
    // Reference: https://github.com/bsoyka/advent-of-code-ocr/blob/main/advent_of_code_ocr/characters.py
    let mut letters = FxHashMap::default();
    letters.insert(String::from(".##.\n#..#\n#..#\n####\n#..#\n#..#"), 'A');
    letters.insert(String::from("###.\n#..#\n###.\n#..#\n#..#\n###."), 'B');
    letters.insert(String::from(".##.\n#..#\n#...\n#...\n#..#\n.##."), 'C');
    letters.insert(String::from("####\n#...\n###.\n#...\n#...\n####"), 'E');
    letters.insert(String::from("####\n#...\n###.\n#...\n#...\n#..."), 'F');
    letters.insert(String::from(".##.\n#..#\n#...\n#.##\n#..#\n.###"), 'G');
    letters.insert(String::from("#..#\n#..#\n####\n#..#\n#..#\n#..#"), 'H');
    letters.insert(String::from(".###\n..#.\n..#.\n..#.\n..#.\n.###"), 'I');
    letters.insert(String::from("..##\n...#\n...#\n...#\n#..#\n.##."), 'J');
    letters.insert(String::from("#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#"), 'K');
    letters.insert(String::from("#...\n#...\n#...\n#...\n#...\n####"), 'L');
    letters.insert(String::from(".##.\n#..#\n#..#\n#..#\n#..#\n.##."), 'O');
    letters.insert(String::from("###.\n#..#\n#..#\n###.\n#...\n#..."), 'P');
    letters.insert(String::from("###.\n#..#\n#..#\n###.\n#.#.\n#..#"), 'R');
    letters.insert(String::from(".###\n#...\n#...\n.##.\n...#\n###."), 'S');
    letters.insert(String::from("#..#\n#..#\n#..#\n#..#\n#..#\n.##."), 'U');
    letters.insert(String::from("#...\n#...\n.#.#\n..#.\n..#.\n..#."), 'Y');
    letters.insert(String::from("####\n...#\n..#.\n.#..\n#...\n####"), 'Z');

    letters.get(letter).copied().unwrap_or('?')
}
